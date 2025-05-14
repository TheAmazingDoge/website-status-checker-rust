use reqwest::blocking::ClientBuilder;
use serde::Serialize;
use std::sync::Mutex;
use chrono::{DateTime, Utc, Duration};
use std::time::{SystemTime, Duration as StdDuration};
use std::thread;

#[derive(Serialize, Clone)]
pub enum WebsiteState {
    Unchecked,
    Response(u16),
    NetworkError(String),
}

#[derive(Serialize, Clone)]
pub struct Website {
    pub url: String,
    pub is_checked: bool,
    pub action_status: WebsiteState,
    pub response_time: Option<f64>,
    pub timestamp: Option<String>, // Central Time timestamp
}

impl Website {
    pub fn create(address: &str) -> Mutex<Self> {
        Mutex::new(Website {
            url: address.to_string(),
            is_checked: false,
            action_status: WebsiteState::Unchecked,
            response_time: None,
            timestamp: None,
        })
    }

    pub fn fetch_status(&mut self, timeout: u64, retries: u32) {
        let client = ClientBuilder::new()
            .timeout(StdDuration::from_secs(timeout))
            .build()
            .expect("Failed to build HTTP client");

        let mut attempts = 0;
        let start_time = SystemTime::now();

        loop {
            self.action_status = match client.head(&self.url).send() {
                Ok(res) => {
                    self.response_time = start_time.elapsed().ok().map(|duration| {
                        let seconds = duration.as_secs() as f64 + duration.subsec_millis() as f64 / 1000.0;
                        (seconds * 100.0).round() / 100.0 // Round to 2 decimal places
                    });

                    // Subtract 6 hours from UTC to get Central Time
                    let utc_time = DateTime::<Utc>::from(SystemTime::now());
                    let central_time = utc_time - Duration::hours(5);
                    self.timestamp = Some(central_time.format("%Y-%m-%d %H:%M:%S").to_string());

                    self.is_checked = true;
                    WebsiteState::Response(res.status().as_u16())
                }
                Err(_) => {
                    attempts += 1;
                    if attempts > retries {
                        self.response_time = start_time.elapsed().ok().map(|duration| {
                            let seconds = duration.as_secs() as f64 + duration.subsec_millis() as f64 / 1000.0;
                            (seconds * 100.0).round() / 100.0
                        });

                        // Subtract 6 hours from UTC for errors
                        let utc_time = DateTime::<Utc>::from(SystemTime::now());
                        let central_time = utc_time - Duration::hours(6);
                        self.timestamp = Some(central_time.format("%Y-%m-%d %H:%M:%S").to_string());

                        self.is_checked = true;
                        self.action_status = WebsiteState::NetworkError("Network error".to_string());
                        break;
                    }
                    thread::sleep(StdDuration::from_millis(100)); // Wait 100 ms before retrying
                    continue;
                }
            };
            break;
        }
    }

    pub fn show_status(&self) {
        match &self.action_status {
            WebsiteState::Unchecked => panic!("'{}' was not checked!", self.url),
            WebsiteState::NetworkError(_) => println!("Error connecting to '{}: Network error'", self.url),
            WebsiteState::Response(status) => {
                println!("Status for '{}': {}", self.url, status);
            }
        }
    }
}