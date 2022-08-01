use anyhow::{anyhow, Result};
use ssh2::Session;
use std::{io::Read, net::TcpStream, str};

pub type CmdOutput = (i32, String);

pub fn connect(username: &str, host: &str, port: u32) -> Result<Session> {
    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let mut sess = Session::new()?;

    sess.set_tcp_stream(tcp);
    sess.handshake()?;
    sess.userauth_agent(&username)?;

    if sess.authenticated() {
        Ok(sess)
    } else {
        Err(anyhow!("session is not authenticated"))
    }
}

pub fn exec(sess: &Session, cmd: &str, handler: fn(&str, &CmdOutput) -> ()) -> Result<CmdOutput> {
    let mut channel = sess.channel_session()?;
    channel.exec(cmd)?;
    let mut output = String::new();
    channel.read_to_string(&mut output)?;
    channel.wait_close()?;
    let exit_code = channel.exit_status()?;
    let cmd_output = (exit_code, output);
    handler(&cmd, &cmd_output);
    Ok(cmd_output)
}

pub fn exec_(sess: &Session, cmd: &str) -> Result<CmdOutput> {
    return exec(sess, cmd, |_, _| ());
}
