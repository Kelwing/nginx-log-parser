/// Contains structures representing Nginx log file lines
use serde::Deserialize;
use std::io::BufRead;

/// Represents a single line in an Nginx log file
#[derive(Debug, Clone, Deserialize)]
pub struct NginxLogLine {
    pub time: String,
    pub remote_ip: String,
    pub remote_user: String,
    pub request: String,
    pub response: u16,
    pub bytes: u64,
    pub referrer: String,
    pub agent: String,
}

/// Represents an entire Nginx log file
#[repr(transparent)]
pub struct NginxLog(pub Vec<NginxLogLine>);

impl NginxLog {
    /// Creates a new NginxLog from a file at the given path
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the Nginx log file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read, or if the file is not valid JSON
    pub fn from_path<P>(path: P) -> Result<Self, Box<dyn std::error::Error>>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);

        let mut log = Self(Vec::new());

        for line in reader.lines() {
            let line = line?;
            let log_line: NginxLogLine = serde_json::from_str(&line)?;
            log.0.push(log_line);
        }

        Ok(log)
    }
}
