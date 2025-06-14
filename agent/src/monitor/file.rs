// This file implements file monitoring capabilities, allowing the agent to observe file changes and access patterns.

use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

pub struct FileMonitor {
    path: String,
    last_modified: SystemTime,
}

impl FileMonitor {
    pub fn new(path: &str) -> Self {
        let metadata = fs::metadata(path).expect("Unable to read metadata");
        let last_modified = metadata.modified().expect("Unable to get last modified time");
        
        FileMonitor {
            path: path.to_string(),
            last_modified,
        }
    }

    pub fn check_for_changes(&mut self) -> bool {
        let metadata = fs::metadata(&self.path).expect("Unable to read metadata");
        let current_modified = metadata.modified().expect("Unable to get last modified time");

        if current_modified > self.last_modified {
            self.last_modified = current_modified;
            true
        } else {
            false
        }
    }

    pub fn get_file_contents(&self) -> String {
        fs::read_to_string(&self.path).expect("Unable to read file")
    }
}

// Example usage
// fn main() {
//     let mut monitor = FileMonitor::new("path/to/file.txt");
//     loop {
//         if monitor.check_for_changes() {
//             println!("File has changed: {}", monitor.get_file_contents());
//         }
//         std::thread::sleep(Duration::from_secs(5));
//     }
// }