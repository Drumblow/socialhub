use std::time::Instant;
use log::info;

pub struct Metrics {
    start_time: Instant,
    endpoint: String,
}

impl Metrics {
    pub fn new(endpoint: &str) -> Self {
        Self {
            start_time: Instant::now(),
            endpoint: endpoint.to_string(),
        }
    }

    pub fn record_response(&self, status_code: u16) {
        let duration = self.start_time.elapsed();
        info!(
            "Endpoint: {}, Status: {}, Duration: {:?}",
            self.endpoint, status_code, duration
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_metrics_recording() {
        let metrics = Metrics::new("/test/endpoint");
        thread::sleep(Duration::from_millis(100));
        metrics.record_response(200);
    }
}
