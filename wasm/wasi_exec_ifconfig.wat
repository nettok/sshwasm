(module
    (import "env" "exec" (func $env_exec (param i32 i32) (result i32 i32 i32 i32)))

    ;; Import the required fd_write WASI function which will write the given io vectors to stdout
    ;; The function signature for fd_write is:
    ;; (File Descriptor, *iovs, iovs_len, nwritten) -> Returns number of bytes written
    (import "wasi_unstable" "fd_write" (func $fd_write (param i32 i32 i32 i32) (result i32)))

    (memory (export "memory") 2)
    (data (i32.const 8) "ifconfig")

    (func $main (export "_start")
        (local $output_ptr i32)
        (local $output_len i32)
        i32.const 8             ;; ptr to start of "ifconfig"
        i32.const 8             ;; len
        call $env_exec
        local.set $output_len
        local.set $output_ptr
        drop    ;; drop output_truncated flag
        drop    ;; drop exit_code

        ;; Creating a new io vector within linear memory
        (i32.store (i32.const 0) (local.get $output_ptr))   ;; iov.iov_base - This is a pointer to the start of the output string
        (i32.store (i32.const 4) (local.get $output_len))   ;; iov.iov_len - The length of the output string

        (call $fd_write
            (i32.const 1) ;; file_descriptor - 1 for stdout
            (i32.const 0) ;; *iovs - The pointer to the iov array, which is stored at memory location 0
            (i32.const 1) ;; iovs_len - We're printing 1 string stored in an iov - so one.
            (i32.const 20) ;; nwritten - A place in memory to store the number of bytes written
        )
        drop ;; Discard the number of bytes written from the top of the stack
    )
)