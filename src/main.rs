use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;
use std::str;

use sshwasm::ssh::connect;
use sshwasm::wasm::run_script;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    destination: String,

    #[clap(value_parser)]
    webassembly: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let (username, host, port) = parse_destination(&cli.destination)?;

    let sess = connect(&username, &host, port)?;
    run_script(&sess, cli.webassembly)?;

    Ok(())
}

fn parse_destination(destination: &str) -> Result<(String, String, u32)> {
    let make_error = || anyhow!("invalid destination");
    let mut split_semicolon = destination.split(':');

    let mut split_at = split_semicolon.next().ok_or_else(make_error)?.split('@');
    let port: u32 = split_semicolon
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(22);

    let username = split_at.next().ok_or_else(make_error)?;
    let host = split_at.next().ok_or_else(make_error)?;

    Ok((username.to_owned(), host.to_owned(), port))
}
