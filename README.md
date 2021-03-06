<div align="center">
   ⛈️🦀
</div>

<h1 align="center">
  Rust Web Template
</h1>

<p align="center">
   Examples of <a href="https://aws.amazon.com/builders-library/challenges-with-distributed-systems/">distributed</a> application concepts to get started with, written in <a href="https://www.rust-lang.org/">Rustlang</a> 
</p>

<div align="center">
  <a alt="GitHub Workflow Status" href="https://github.com/davidmaceachern/rust-web-template/actions">
    <img  src="https://img.shields.io/github/workflow/status/davidmaceachern/rust-web-template/Rust">
  </a>
</div>
<br />

## Features

- `Collector` fetches data from the [Phemex Exchange](https://phemex.com/) and saves it to
  a [Minio](https://min.io/) [S3](https://aws.amazon.com/s3/) bucket.
- `Price-checker` lists and prints count of objects in the bucket.

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
|-- collector             <--- Logic for collecting data from an external source.
|   |-- Cargo.toml        <--- Where we can define collector dependencies.
|   `-- src
|       `-- main.rs       <--- Binary containing the collector logic.
|       `-- phemex.rs     <--- Module containing the domain types.
|       `-- repository.rs <--- Module containing the Minio (s3) logic.
|-- infrastructure/       <--- Example of how to deploy to Kubernetes
|-- price-checker         <--- Checks the previous prices for any patterns 
|   |-- Cargo.toml
|   `-- src
|      `-- main.rs
|      `-- repository.rs
|-- README.md             <--- This file that you are reading.
|-- run-dynamodb.sh       <--- Bash script to run a local Dynamodb container.
|-- run-s3.sh             <--- Bash script to run a local Minio (S3) container.
|-- run-e2e.sh            <--- Bash script to run the collector and then the price checker.
```

## Development

In separate processes/terminal window run the services that we will be developing against.

To run the s3bucket server.

`$ bash run-s3.sh`

To run the Rust application.

`$ bash run-e2e.sh`

`$ bash run-dynamodb.sh` <----------------- todo

Run syntax checking on client code as we develop by using `cd collector && cargo watch -s 'cargo build'`

Run the client.

`$ cargo run --bin client`

Run the collector.
`$ cargo run --bin collector`

Install new packages using `cargo add`.

Format code using `cargo fmt`.

## Continuous Integration

### Github Actions

Two actions are included:

- `docker-publish` which currently publishes the collector container to the Github Container Registry whenever commits are pushed to the repository, read more [here](https://dev.to/davidmaceachern/how-to-fix-github-docker-containers-built-with-actions-162k).
- `general` which runs `rust fmt` to keep the Rust code to commonly accepted standard upon pushing commits, fix issues with this build by running cargo fmt before commiting code.

## Deployment

Kubernetes enables deployment to any provider that has Kubernetes installed. The following steps assume that a cluster has been installed.

1. Check that a node is running on the cluster `$ kubectl get node`.
2. Claim some data from the host system for Minio `$ kubectl create -f ./infrastructure/persistent-volume-claim.yaml`.
3. Create the deployment `kubectl create -f ./infrastructure/deployment.yaml`.
4. Create the Minio service `kubectl create -f ./infrastructure/service.yaml`.
5. List to see what exists `kubectl get all`, or view the Minio access panel in the browser `http://127.0.0.1:9000/minio/login`.
6. Delete `kubectl delete all -l example=rust-web-template`

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
