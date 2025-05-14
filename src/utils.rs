use crate::site::Website;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Mutex;

pub fn read_sites(file_path: &str) -> Vec<Mutex<Website>> {
    let file = File::open(file_path).expect("Couldn't open file");
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .map(|address| Website::create(&address))
        .collect()
}