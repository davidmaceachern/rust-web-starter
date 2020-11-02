# Rust Web Template

Examples of web/distributed application concepts to get started with, written in
Rust.

## Features

- Uses the Tokio runtime to collect data from the Phemex Exchange and save it to
  an Minio (s3) bucket.

## Structure

```
|-- Cargo.toml            <--- Where we can define application dependencies.
|-- client                <--- Logic for tests and the app to interface with infrastructure.
|   |-- Cargo.toml        <--- Where we can define client dependencies.
|   `-- src
|       `-- main.rs       <--- Binary containing the client logic.
|-- readme.md
|-- run-dynamodb.sh       <--- Bash script to run a local Dynamodb container.
|-- run-s3.sh             <--- Bash script to run a local Minio (S3) container.
`-- src                   <--- Rust-web-template source, our main business logic.
    `-- main.rs           <--- The binary containing the application logic.
```

## Development

In seperate processes/terminal window run the services that we will be developing against.

`$ bash run-s3.sh`

`$ bash run-dynamodb.sh` <----------------- todo

Begin by interacting with these services using the code in `/client/src/main.rs`.

Run client code as we develop by using `cd client && cargo watch -s 'cargo build'`

Install new packages using `cargo add`.

Format code using `cargo fmt`.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
