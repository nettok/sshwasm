(module
    (import "env" "exec" (func $env_exec (param i32 i32) (result i32 i32 i32 i32)))
    (import "env" "log_output" (func $env_log_output (param i32 i32 i32 i32)))

    (memory (export "memory") 2)
    (data (i32.const 0) "ifconfig")

    (func $main (export "_start")
        i32.const 0                 ;; ptr to start of "ifconfig"
        i32.const 8                 ;; len
        call $env_exec
        call $env_log_output)
)
