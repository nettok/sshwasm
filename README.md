# sshwasm
SSH into a machine and execute commands scripted with WebAssembly

## Usage
```sh
sshwasm 0.1.0
SSH into a machine and execute commands scripted with WebAssembly

USAGE:
    sshwasm <DESTINATION> <WEBASSEMBLY>

ARGS:
    <DESTINATION>    Destination host to SSH into (example: username@hostname:22)
    <WEBASSEMBLY>    WebAssembly file (format can be .wasm or .wat)

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

## How it works
*sshwasm* first connects to the destination using SSH, and then locally runs the specified WebAssembly program.

*sshwasm* exposes a couple of functions to the embedded WebAssembly program while connected to the remote machine:
- `exec` executes a command in the remote machine
- `log_output` locally prints the output of the remotely executed command

The WebAssembly program also has access to [WASI](https://wasi.dev/) functionality, e.g. it is able to do system calls.


## Examples

### exec_ifconfig.wat
Runs `ifconfig` in the remote machine, and prints the output in the screen using a *sshwasm* provided function.

See code: [wasm/exec_ifconfig.wat](wasm/exec_ifconfig.wat)

### wasi_exec_ifconfig.wat
It does the same as the `exec_ifconfig.wat`, but it uses the WASI `fd_write` function to print the output of the command to the screen.

See code: [wasm/wasi_exec_ifconfig.wat](wasm/wasi_exec_ifconfig.wat)
