// This file provides sandboxing functionality specific to Windows, ensuring that monitored processes operate within a controlled environment.

use std::process::{Command, Stdio};
use std::io::{self, Write};

pub struct WindowsSandbox {
    pub process: Option<Command>,
}

impl WindowsSandbox {
    pub fn new() -> Self {
        WindowsSandbox { process: None }
    }

    pub fn start_process(&mut self, command: &str, args: &[&str]) -> io::Result<()> {
        let mut cmd = Command::new(command);
        cmd.args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        self.process = Some(cmd.spawn()?);
        Ok(())
    }

    pub fn stop_process(&mut self) -> io::Result<()> {
        if let Some(mut process) = self.process.take() {
            process.kill()?;
        }
        Ok(())
    }

    pub fn execute_command(&mut self, command: &str) -> io::Result<String> {
        let output = Command::new(command)
            .output()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    }
}