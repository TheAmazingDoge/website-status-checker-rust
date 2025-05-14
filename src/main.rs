mod config;
mod site;
mod worker;
mod utils;

use std::fs::File;
use std::sync::Arc;
use serde_json::to_writer_pretty;
use worker::launch_threads;

fn main() {
    let (input_file, max_workers, timeout, retries) = config::parse_arguments();

    let website_list = Arc::new(utils::read_sites(&input_file));

    // Start worker threads
    launch_threads(website_list.clone(), max_workers, timeout, retries);

    // Collect results
    let results: Vec<_> = website_list
        .iter()
        .map(|website| {
            let website = website.lock().unwrap();
            website.clone()
        })
        .collect();

    let file = File::create("status.json").expect("Couldn't create file");
    to_writer_pretty(file, &results).expect("Couldn't write to file");

    // Output statuses
    for website in website_list.iter() {
        let website = website.lock().unwrap();
        website.show_status();
    }
}