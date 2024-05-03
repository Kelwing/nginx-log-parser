use std::fmt::Display;

use crate::nginx_log::NginxLog;

/// Represents statistics about an Nginx log
///
/// # Contains
///
/// * `status_count` - A map of status codes to the number of times they were returned
/// * `mean_all_requests` - The mean number of bytes returned for all requests
/// * `mean_successful_requests` - The mean number of bytes returned for successful requests
/// * `mean_failed_requests` - The mean number of bytes returned for failed requests
/// * `median_all_requests` - The median number of bytes returned for all requests
/// * `median_successful_requests` - The median number of bytes returned for successful requests
/// * `median_failed_requests` - The median number of bytes returned for failed requests
/// * `p99_all_requests` - The 99th percentile of bytes returned for all requests
/// * `p99_successful_requests` - The 99th percentile of bytes returned for successful requests
/// * `p99_failed_requests` - The 99th percentile of bytes returned for failed requests
/// * `largest_endpoint` - The enpoint that returned the largest response
/// * `failingest_endpoint` - The endpoint that returned the most failed responses
pub struct LogStats {
    pub status_count: std::collections::BTreeMap<u16, usize>,
    pub mean_all_requests: f64,
    pub mean_successful_requests: f64,
    pub mean_failed_requests: f64,
    pub median_all_requests: f64,
    pub median_successful_requests: f64,
    pub median_failed_requests: f64,
    pub p99_all_requests: f64,
    pub p99_successful_requests: f64,
    pub p99_failed_requests: f64,
    pub largest_endpoint: String,
    pub failingest_endpoint: String,
}

impl LogStats {
    /// Creates a new LogStats from an NginxLog
    ///
    /// # Arguments
    ///
    /// * `log` - The NginxLog to generate statistics from
    ///
    /// # Returns
    ///
    /// A new LogStats instance
    pub fn from_nginx_log(log: &NginxLog) -> Self {
        let mut status_count = std::collections::BTreeMap::new();
        let mut largest_endpoint = ("".to_owned(), 0);
        let mut endpoint_failures = std::collections::BTreeMap::new();
        let mut all_bytes = Vec::new();

        for line in &log.0 {
            let endpoint = line.request.split_whitespace().nth(1).unwrap_or("/");
            *status_count.entry(line.response).or_insert(0) += 1;
            all_bytes.push(line.bytes);

            if line.response >= 400 {
                *endpoint_failures.entry(endpoint.to_owned()).or_insert(0) += 1;
            }

            if line.bytes > largest_endpoint.1 {
                largest_endpoint = (endpoint.to_owned(), line.bytes);
            }
        }
        all_bytes.sort();
        let mut failed_bytes = log
            .0
            .iter()
            .filter(|line| line.response >= 400)
            .map(|line| line.bytes)
            .collect::<Vec<u64>>();
        failed_bytes.sort();
        let mut success_bytes = log
            .0
            .iter()
            .filter(|line| line.response < 400)
            .map(|line| line.bytes)
            .collect::<Vec<u64>>();
        success_bytes.sort();

        Self {
            status_count,
            mean_all_requests: all_bytes.iter().sum::<u64>() as f64 / all_bytes.len() as f64,
            mean_successful_requests: success_bytes.iter().sum::<u64>() as f64
                / success_bytes.len() as f64,
            mean_failed_requests: failed_bytes.iter().sum::<u64>() as f64
                / failed_bytes.len() as f64,
            median_all_requests: all_bytes[all_bytes.len() / 2] as f64,
            median_successful_requests: success_bytes[success_bytes.len() / 2] as f64,
            median_failed_requests: failed_bytes[failed_bytes.len() / 2] as f64,
            p99_all_requests: all_bytes[(all_bytes.len() as f64 * 0.99) as usize] as f64,
            p99_successful_requests: success_bytes[(success_bytes.len() as f64 * 0.99) as usize]
                as f64,
            p99_failed_requests: failed_bytes[(failed_bytes.len() as f64 * 0.99) as usize] as f64,
            largest_endpoint: largest_endpoint.0,
            failingest_endpoint: endpoint_failures
                .iter()
                .max_by_key(|&(_, count)| count)
                .map(|(endpoint, _)| endpoint.to_owned())
                .unwrap_or("/".to_owned()),
        }
    }
}

impl Display for LogStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Status Codes:")?;
        for (status, count) in &self.status_count {
            writeln!(f, "  {}: {}", status, count)?;
        }

        writeln!(f, "Mean Bytes:")?;
        writeln!(f, "  All Requests: {:.2}", self.mean_all_requests)?;
        writeln!(
            f,
            "  Successful Requests: {:.2}",
            self.mean_successful_requests
        )?;
        writeln!(f, "  Failed Requests: {:.2}", self.mean_failed_requests)?;

        writeln!(f, "Median Bytes:")?;
        writeln!(f, "  All Requests: {}", self.median_all_requests)?;
        writeln!(
            f,
            "  Successful Requests: {}",
            self.median_successful_requests
        )?;
        writeln!(f, "  Failed Requests: {}", self.median_failed_requests)?;

        writeln!(f, "99th Percentile Bytes:")?;
        writeln!(f, "  All Requests: {}", self.p99_all_requests)?;
        writeln!(f, "  Successful Requests: {}", self.p99_successful_requests)?;
        writeln!(f, "  Failed Requests: {}", self.p99_failed_requests)?;

        writeln!(f, "Largest Endpoint: {}", self.largest_endpoint)?;
        writeln!(f, "Failingest Endpoint: {}", self.failingest_endpoint)?;

        Ok(())
    }
}
