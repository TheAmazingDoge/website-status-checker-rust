use clap::{Arg, Command};

pub fn parse_arguments() -> (String, usize, u64, u32) {
    let args = Command::new("Website Status Checker")
        .version("1.0")
        .author("Edward Cruz")
        .about("Checks server responses for a list of URLs")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .help("File containing a list of URLs")
                .default_value("sites.txt"),
        )
        .arg(
            Arg::new("workers")
                .short('w')
                .long("workers")
                .value_name("WORKERS")
                .help("Number of simultaneous requests")
                .default_value("2"),
        )
        .arg(
            Arg::new("timeout")
                .short('t')
                .long("timeout")
                .value_name("TIMEOUT")
                .help("Per-request timeout in seconds")
                .default_value("5"),
        )
        .arg(
            Arg::new("retries")
                .short('r')
                .long("retries")
                .value_name("RETRIES")
                .help("Number of retries after a failure")
                .default_value("0"),
        )
        .get_matches();

    let input_file = args.get_one::<String>("file").unwrap().to_string();
    let max_workers = args
        .get_one::<String>("workers")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let timeout = args
        .get_one::<String>("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let retries = args
        .get_one::<String>("retries")
        .unwrap()
        .parse::<u32>()
        .unwrap();

    (input_file, max_workers, timeout, retries)
}