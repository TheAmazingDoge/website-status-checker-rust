use crate::site::Website;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn launch_threads(websites: Arc<Vec<Mutex<Website>>>, thread_count: usize, timeout: u64, retries: u32) {
    let threads: Vec<_> = (0..thread_count)
        .map(|_| {
            let websites = websites.clone();
            thread::spawn(move || {
                for website in websites.iter() {
                    let mut website = match website.try_lock() {
                        Ok(website) => website,
                        Err(_) => continue,
                    };

                    if !website.is_checked {
                        website.fetch_status(timeout, retries);
                    }
                }
            })
        })
        .collect();

    for thread in threads {
        thread.join().unwrap();
    }
}