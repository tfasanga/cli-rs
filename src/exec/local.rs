use std::process::Command;
use std::str::{from_utf8, Utf8Error};

use crate::exec::{CmdError, CmdResult};
use crate::exec::CmdError::NoExitCode;
use crate::exec::ExitCode;

/// # Errors
///
/// Will return `Err` if it was unable to execute the command.
pub fn execute_cmd(cmd: &str, args: &[&str]) -> CmdResult<(ExitCode, String)> {
    let output = Command::new(cmd)
        .args(args)
        .output()?;

    let out_str = from_utf8(&output.stdout)?;
    let result = out_str.to_owned();

    let err_str = from_utf8(&output.stderr)?;
    if err_str.is_empty() {
        eprintln!("Error:\n{err_str}");
    }

    let status = &output.status;
    if status.success() {
        Ok((ExitCode::ExitSuccess, result))
    } else {
        match status.code() {
            None => Err(NoExitCode),
            Some(rc) => Ok((ExitCode::ExitFailure(rc), result)),
        }
    }
}

/// # Errors
///
/// Will return `Err` if it was unable to execute the command.
pub fn run_cmd(cmd: &str, args: &[&str]) -> CmdResult<ExitCode> {
    let mut child = Command::new(cmd)
        .args(args)
        .spawn()?;

    let status = child.wait()?;

    if status.success() {
        Ok(ExitCode::ExitSuccess)
    } else {
        match status.code() {
            None => Err(NoExitCode),
            Some(rc) => Ok(ExitCode::ExitFailure(rc)),
        }
    }
}

impl From<Utf8Error> for CmdError {
    fn from(e: Utf8Error) -> Self {
        CmdError::Utf8Error(e.to_string())
    }
}
