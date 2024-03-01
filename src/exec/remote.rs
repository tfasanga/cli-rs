use std::io::Read;
use std::net::TcpStream;
use std::path::Path;

use ssh2::{Error, ErrorCode, Session};

use crate::exec::{CmdError, CmdResult};
use crate::exec::ExitCode;
use crate::exec::ssh::SshCredentials;

/// # Errors
///
/// Will return `Err` if it was unable to execute the command.
pub fn execute_cmd(creds: &SshCredentials, cmd: &str, args: &[&str]) -> CmdResult<(ExitCode, String)> {
    let host = creds.hostname.clone();
    let port = creds.port.unwrap_or(22);
    let tcp = TcpStream::connect((host, port))?;

    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake().map_err(map_handshake_err)?;

    authenticate(&session, creds).map_err(map_auth_err)?;

    if session.authenticated() {
        let command = to_command_str(cmd, args);

        let mut channel = session.channel_session()?;
        let mut result = String::new();

        channel.exec(command.as_str())?;
        channel.read_to_string(&mut result)?;
        channel.wait_close()?;

        let rc = channel.exit_status()?;
        let ec = ExitCode::from_rc(rc);
        Ok((ec, result))
    } else {
        Err(CmdError::Authentication("not authenticated".to_owned()))
    }
}

/// # Errors
///
/// Will return `Err` if it was unable to execute the command.
pub fn run_cmd(creds: &SshCredentials, cmd: &str, args: &[&str]) -> CmdResult<ExitCode> {
    let host = creds.hostname.clone();
    let port = creds.port.unwrap_or(22);
    let tcp = TcpStream::connect((host, port))?;

    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake().map_err(map_handshake_err)?;

    authenticate(&session, creds).map_err(map_auth_err)?;

    if session.authenticated() {
        let mut result = String::new();
        let mut channel = session.channel_session()?;

        let command = to_command_str(cmd, args);

        channel.exec(command.as_str())?;
        channel.read_to_string(&mut result)?;
        channel.wait_close()?;

        let rc = channel.exit_status()?;
        let ec = ExitCode::from_rc(rc);
        println!("{result}"); // TODO
        Ok(ec)
    } else {
        Err(CmdError::Authentication("not authenticated".to_owned()))
    }
}

fn authenticate(session: &Session, creds: &SshCredentials) -> Result<(), Error> {
    if let Some(key_file) = &creds.private_key_file {
        let pubkey = creds.public_key_file.as_ref().map(Path::new);
        let privatekey = Path::new(key_file);
        let username = creds.username.as_str();

        session.userauth_pubkey_file(
            username,
            pubkey,
            privatekey,
            None,
        )
    } else if let Some(password) = &creds.password {
        let username = creds.username.as_str();

        session.userauth_password(username, password.as_str())
    } else {
        Err(Error::new(ErrorCode::Session(0), "no authentication method"))
    }
}

fn to_command_str(cmd: &str, args: &[&str]) -> String {
    if args.is_empty() {
        cmd.to_owned()
    } else {
        let args_str = args.join(" ");
        format!("{cmd} {args_str}")
    }
}

impl From<Error> for CmdError {
    fn from(e: Error) -> Self {
        CmdError::Ssh(e.message().to_owned())
    }
}

#[allow(clippy::needless_pass_by_value)]
fn map_auth_err(e: Error) -> CmdError {
    CmdError::Authentication(e.message().to_owned())
}

#[allow(clippy::needless_pass_by_value)]
fn map_handshake_err(e: Error) -> CmdError {
    CmdError::Handshake(e.message().to_owned())
}
