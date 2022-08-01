use anyhow::Result;
use ssh2::Session;
use std::path::Path;
use std::str;
use wasmtime::*;
use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

use super::ssh::exec_;

struct State {
    session: Session,
    wasi: WasiCtx,
}

pub fn run_script(sess: &Session, file: impl AsRef<Path>) -> Result<()> {
    let engine = Engine::new(Config::new().debug_info(true))?;
    let module = Module::from_file(&engine, file)?;
    let mut linker = Linker::new(&engine);

    wasmtime_wasi::add_to_linker(&mut linker, |state: &mut State| &mut state.wasi)?;

    linker.func_wrap("env", "exec", env_exec)?;
    linker.func_wrap("env", "log_output", env_log_output)?;

    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();

    let state = State {
        session: sess.to_owned(),
        wasi,
    };
    let mut store = Store::new(&engine, state);
    let instance = linker.instantiate(&mut store, &module)?;
    let start = instance.get_typed_func::<(), (), _>(&mut store, "_start")?;

    start.call(&mut store, ())?;

    Ok(())
}

fn read_string_from_memory(
    store_context: impl AsContext,
    memory: &Memory,
    ptr: usize,
    len: usize,
) -> String {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize);
    buffer.resize(len as usize, 0);
    memory
        .read(store_context, ptr as usize, &mut buffer)
        .unwrap();
    str::from_utf8(&buffer).unwrap().to_owned()
}

// Exported host functions

fn env_exec(mut caller: Caller<'_, State>, ptr: i32, len: i32) -> (i32, i32, i32, i32) {
    let memory: Memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let cmd = read_string_from_memory(caller.as_context(), &memory, ptr as usize, len as usize);

    let (exit_code, mut output) = exec_(&caller.data().session, &cmd).unwrap();

    let page_size = 64_000;
    let max_output_len = page_size;
    let mut output_truncated = 0;
    if output.len() > max_output_len {
        output.truncate(max_output_len);
        output_truncated = 1;
    }

    let output_ptr = page_size * 1;
    let output_len = output.len();
    memory
        .write(caller.as_context_mut(), output_ptr, output.as_bytes())
        .unwrap();

    (
        exit_code,
        output_truncated,
        output_ptr as i32,
        output_len as i32,
    )
}

fn env_log_output(
    mut caller: Caller<'_, State>,
    exit_code: i32,
    output_truncated: i32,
    ptr: i32,
    len: i32,
) {
    let memory: Memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let output = read_string_from_memory(caller.as_context(), &memory, ptr as usize, len as usize);

    print!("exit_code = {}", exit_code);
    if output_truncated > 0 {
        print!(", output_truncated = true");
    }
    println!();
    println!("{}", output)
}
