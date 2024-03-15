use crate::exec::{CmdResult, CommandExecutor, ExitCode, local, remote};
use crate::exec::ssh::SshCredentials;

#[derive(Debug, PartialEq, Clone)]
pub enum Machine {
    LocalMachine,
    RemoteMachine(SshCredentials),
}

impl std::fmt::Display for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Machine::LocalMachine => { "localhost".fmt(f) }
            Machine::RemoteMachine(creds) => {
                let hostname = &creds.hostname;
                if let Some(port) = &creds.port {
                    format!("{hostname}:{port}").fmt(f)
                } else {
                    hostname.fmt(f)
                }
            }
        }
    }
}

impl CommandExecutor for Machine {
    fn execute_cmd(&self, cmd: &str, args: &[&str]) -> CmdResult<(ExitCode, String)> {
        match &self {
            Machine::LocalMachine => local::execute_cmd(cmd, args),
            Machine::RemoteMachine(ssh_creds) => remote::execute_cmd(ssh_creds, cmd, args)
        }
    }

    fn run_cmd(&self, cmd: &str, args: &[&str]) -> CmdResult<ExitCode> {
        match &self {
            Machine::LocalMachine => local::run_cmd(cmd, args),
            Machine::RemoteMachine(ssh_creds) => remote::run_cmd(ssh_creds, cmd, args),
        }
    }
}

impl Machine {
    #[must_use] pub fn is_local(&self) -> bool {
        match self {
            Machine::LocalMachine => true,
            Machine::RemoteMachine(_) => false,
        }
    }

    #[must_use] pub fn hostname(&self) -> &str {
        match self {
            Machine::LocalMachine => "localhost",
            Machine::RemoteMachine(creds) => &creds.hostname,
        }
    }
}
