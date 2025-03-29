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
