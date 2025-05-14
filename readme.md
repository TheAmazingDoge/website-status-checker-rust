# Website Status Checker
A Rust-based command-line tool to check the status of websites from a list of URLs. The tool supports concurrent requests, configurable timeouts, retries, and outputs the results in JSON format.

## Features
- **Concurrent Requests**: Specify the number of worker threads to handle multiple requests simultaneously.  
- **Configurable Timeout**: Set a timeout for each request to avoid hanging on unresponsive websites.  
- **Retries**: Retry failed requests a configurable number of times with a 100 ms delay between attempts.  
- **Batch Output**: Outputs the results to a status.json file, including:
    - URL
    - HTTP status code or error message
    - Response time (in seconds, rounded to 2 decimal places)
    - Timestamp of the request  

- **Command-Line Arguments**: Flexible configuration using command-line flags.  

## Build Instructions
1. **Clone the Repository**
```
git clone https://github.com/TheAmazingDoge/website-status-checker-rust.git
cd website-status-checker-rust
```

2. **Build the Release Version**
```
cargo build --release
```

3. **Locate the Executable** The compiled executable will be located in the `target/release` directory:
```
cd target/release
```

## Usage
Run the program from the terminal with the following syntax:
```
./website-status-checker-rust [--file sites.txt][--workers N][--timeout S][--retries N]
```

**Command-Line Arguments**
|Argument|	Short|	Description|	Default Value|
|:-----|:--------:|:--------:|------:|
|`--file`|	`-f`|	File containing a list of URLs (one per line).|	`urls.txt`|
|`--workers`|	`-w`|	Number of simultaneous requests (threads).|	`2`|
|`--timeout`|	`-t`|	Per-request timeout in seconds.|	`5`|
|`--retries`|	`-r`|	Number of retries after a failure.|	`0`|

**Examples**
1. **Using a File with Default Settings**
```
./website-status-checker-rust --file urls.txt
```
2. **Specifying Workers and Timeout**
```
./website-status-checker-rust --file urls.txt --workers 4 --timeout 10
```
3. **Adding Retries**
```
./website-status-checker-rust --file urls.txt --workers 4 --timeout 10 --retries 3
```
4. **Using a Custom File**
```
./website-status-checker-rust --file custom_urls.txt --workers 8 --timeout 15 --retries 5
```
## Output
The program outputs the results to a `status.json` file in the current working directory. The JSON file contains an array of objects with the following structure:
```
[
  {
    "url": "https://google.com",
    "is_checked": true,
    "action_status": {
      "Response": 200
    },
    "response_time": 0.36,
    "timestamp": "2025-05-14 01:56:12"
  },
  {
    "url": "https://rust-lang.org",
    "is_checked": true,
    "action_status": {
      "Response": 200
    },
    "response_time": 0.59,
    "timestamp": "2025-05-14 01:56:12"
  },
  {
    "url": "https://somerandomlinkthatshouldnotwork.com",
    "is_checked": true,
    "action_status": {
      "NetworkError": "Network error"
    },
    "response_time": 0.37,
    "timestamp": "2025-05-14 01:56:12"
  }
]
```

## Features in Detail
1. **Concurrent Requests**
- Use the `--workers` argument to specify the number of threads for concurrent requests.
- Example:
```
./website-status-checker-rust --file urls.txt --workers 4
```
2. **Configurable Timeout**
- Use the `--timeout` argument to set the maximum time (in seconds) for each request.
- Example:
```
./website-status-checker-rust --file urls.txt --timeout 10
```
3. **Retries**
- Use the `--retries` argument to specify the number of retries for failed requests.
- Example:
```
./website-status-checker-rust --file urls.txt --retries 3
```
4. **Batch Output**
- Results are saved to `status.json` in the current working directory.
- Includes:
    - URL
    - HTTP status code or error message
    - Response time (in seconds)
    - Timestamp of the request

## Error Handling
- If the `--file` argument is not supplied or the file is empty, the program will print:
```
Error: You must supply a --file.
```
and exit with code `2`.

- If the file cannot be found, the program will print:
```
Couldn't open file: <file>
```
