<div align="center">
   ⛈️🦀
</div>

<h1 align="center">
  Rust Web Template
</h1>

<p align="center">
   Examples of <a href="https://aws.amazon.com/builders-library/challenges-with-distributed-systems/">distributed</a> application concepts to get started with, written in <a href="https://www.rust-lang.org/">Rustlang</a> 
</p>

<br />
<div align="center">
  <a alt="GitHub Actions" href="https://github.com/softprops/devtogo/actions">
    <img src="https://github.com/softprops/devtogo/workflows/Main/badge.svg"/>
  </a>
</div>

<br />

## Features

- Uses the [Tokio](https://tokio.rs/) runtime to collect data from the [Phemex Exchange](https://phemex.com/) and save it to
  a [Minio](https://min.io/) [S3](https://aws.amazon.com/s3/) bucket.

## Goals

- Build a batch process, which is considered to be a "offline distributed system" as opposed to a real time analytics pipeline.
- Attempt to map existing knowledge into a Rust application, consolidate skills and test Rust's readiness for this level of the stack.
- Avoid being too opinionated about where it will be deployed.
- Evolve to a state which it can be used to build out other ideas that I have.

## Structure

```
|-- Cargo.toml            <--- Where we can define our workspace configuration.
|-- client                <--- Logic for the app and tests to interface with infrastructure.
|   |-- Cargo.toml        <--- Where we can define client dependencies.
|   `-- src
|       `-- main.rs       <--- Binary containing the client logic.
|-- collector             <--- Logic for tests and the app to interface with infrastructure.
|   |-- Cargo.toml        <--- Where we can define collector dependencies.
|   `-- src
|       `-- main.rs       <--- Binary containing the collector logic.
|-- readme.md
|-- run-dynamodb.sh       <--- Bash script to run a local Dynamodb container.
|-- run-s3.sh             <--- Bash script to run a local Minio (S3) container.
```

## Development

In seperate processes/terminal window run the services that we will be developing against.

`$ bash run-s3.sh`

`$ bash run-dynamodb.sh` <----------------- todo

Begin by interacting with these services using the code in `/client/src/main.rs`.

Run syntax checking on client code as we develop by using `cd collector && cargo watch -s 'cargo build'`

Run the client.

`$ cargo run --bin client`

Run the collector.
`$ cargo run --bin collector`

Install new packages using `cargo add`.

Format code using `cargo fmt`.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
