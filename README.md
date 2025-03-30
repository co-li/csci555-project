# CSCI 555 Project

Rust-based Web Servers Benchmarking

## Before Commit to Repo

> [!CAUTION]
> Use the following command to delete cargo build files before `git commit`!

`cargo clean`

## Web Server Directories

- Baseline: {Repo}/baseline
- Rocket: {Repo}/server-rocket

## How to Run

`cd {Repo}/{Web Server Directory}`

`cargo run`

## Default URL

- Baseline: 127.0.0.1:7000
- Rocket: 127.0.0.1:8000

## Server Routes

- /plaintext: Plain text "Hello, world!"
- /json: JSON Object {"message": "Hello, world!"}

## monitor.py
- How to run?
```
usage: monitor.py [-h] [--args [ARGS ...]]
                  [--interval INTERVAL]
                  [--output-dir OUTPUT_DIR] [--attach ATTACH]
                  [--port PORT]
                  binary_path

Monitor resource usage of a binary

positional arguments:
  binary_path           Path to the binary to monitor

options:
  -h, --help            show this help message and exit
  --args [ARGS ...]     Arguments to pass to the binary
  --interval INTERVAL   Monitoring interval in seconds
  --output-dir OUTPUT_DIR
                        Directory to save monitoring data
  --attach ATTACH       Attach to existing process ID instead
                        of launching new binary
  --port PORT           Port to listen for termination
                        messages
```

- How to stop remotely
```
# server side
python monitor.py binary_path --port ${PORT}

# client side
nc ${SERVER_ADDRESS} ${PORT}
```