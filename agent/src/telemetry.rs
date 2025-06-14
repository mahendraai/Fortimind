use std::fs::File;
use std::io::{self, Write};
use std::time::SystemTime;

pub struct Telemetry {
    log_file: File,
}

impl Telemetry {
    pub fn new(log_file_path: &str) -> io::Result<Self> {
        let log_file = File::create(log_file_path)?;
        Ok(Telemetry { log_file })
    }

    pub fn log_event(&mut self, event: &str) -> io::Result<()> {
        let timestamp = SystemTime::now();
        writeln!(self.log_file, "[{:?}] {}", timestamp, event)?;
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.log_file.flush()
    }
}