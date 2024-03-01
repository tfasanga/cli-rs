use std::borrow::Cow;
use thiserror::Error;
use crate::exec::ExitCode::ExitSuccess;

use crate::machine::Machine;

pub mod ssh;
pub mod local;
pub mod remote;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("connection failed")]
    Connection(String),
    #[error("ssh authentication")]
    Authentication(String),
    #[error("ssh handshake")]
    Handshake(String),
    #[error("execute failed")]
    Execute(String),
    #[error("IO error")]
    Io(String),
    #[error("SSH error")]
    Ssh(String),
    #[error("convert UTF-8")]
    Utf8Error(String),
    #[error("no exit code")]
    NoExitCode,
    #[error("unknown error")]
    Unknown,
}

impl From<std::io::Error> for CmdError {
    fn from(e: std::io::Error) -> Self {
        CmdError::Io(e.to_string())
    }
}

pub type CmdResult<T> = Result<T, CmdError>;

#[derive(Debug)]
pub enum ExitCode {
    ExitSuccess,
    ExitFailure(i32),
}

impl ExitCode {
    fn from_rc(rc: i32) -> ExitCode {
        if rc == 0 {
            ExitCode::ExitSuccess
        } else {
            ExitCode::ExitFailure(rc)
        }
    }
}

pub trait CommandExecutor {
    /// # Errors
    ///
    /// Will return `Err` if it was unable to execute the command.
    fn execute_cmd(&self, cmd: &str, args: &[&str]) -> CmdResult<(ExitCode, String)>;

    /// # Errors
    ///
    /// Will return `Err` if it was unable to execute the command.
    fn run_cmd(&self, cmd: &str, args: &[&str]) -> CmdResult<ExitCode>;
}

/// # Errors
///
/// Will return `Err` if it was unable to execute the command.
pub fn scp(src_m: &Machine, src_file: &str, dst_m: &Machine, dst_file: &str) -> CmdResult<ExitCode> {

    if *src_m.hostname() == *dst_m.hostname() {
        println!("skipping, src and dst machines are the same");
        return Ok(ExitSuccess);
    }

    let exec_machine = match dst_m {
        Machine::LocalMachine => dst_m,
        Machine::RemoteMachine(_) => src_m,
    };

    let source = build_scp_arg(src_m, src_file, exec_machine);
    let destination = build_scp_arg(dst_m, dst_file, exec_machine);

    let args: Vec<&str> = vec![&source, &destination];

    exec_machine.run_cmd("scp", &args)
}

fn build_scp_arg<'a>(machine: &Machine, file: &'a str, exec_machine: &Machine) -> Cow<'a, str> {
    match machine {
        Machine::LocalMachine => Cow::Borrowed(file),
        Machine::RemoteMachine(_) if *machine == *exec_machine => Cow::Borrowed(file),
        Machine::RemoteMachine(creds) => {
            let user = &creds.username;
            let ip = &creds.hostname;
            Cow::Owned(format!("{user}@{ip}/{file}"))
        }
    }
}

/// # Errors
///
/// Will return `Err` if it was unable to execute the command.
pub fn mkdirs(machine: &Machine, dir_name: &str) -> CmdResult<ExitCode> {
    machine.run_cmd("mkdir", &["-p", dir_name])
}

/// # Errors
///
/// Will return `Err` if it was unable to execute the command.
pub fn file_exists(machine: &Machine,file_name: &str) -> CmdResult<bool> {
    file_test(machine, "-f", file_name)
}

/// # Errors
///
/// Will return `Err` if it was unable to execute the command.
pub fn directory_exists(machine: &Machine,file_name: &str) -> CmdResult<bool> {
    file_test(machine, "-d", file_name)
}

fn file_test(machine: &Machine, file_name: &str, option: &str) -> CmdResult<bool> {
    let rc = machine.run_cmd("test", &[option, file_name])?;

    match rc {
        ExitSuccess => Ok(true),
        ExitCode::ExitFailure(_) => Ok(false),
    }
}
