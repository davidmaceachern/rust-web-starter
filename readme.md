# Rust Web Template

Examples of web/distributed application concepts to get started with, written in
some knowledge.

## Development

In seperate processes/terminal window run the services that we will be developing against.

`$ bash run-s3.sh`

`$ bash run-dynamodb.sh`

Begin by interacting with these services using the code in `/client/src/main.rs`.

Run client code as we develop by using `cd client && cargo watch -s 'cargo build'`

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
