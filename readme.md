# Rust Web Template

Examples of web application concepts to get started with, written in Rust to demonstrate some knowledge.

- Pretty-printed Logging with `Femme`.
- Error Handling Pattern with `Thiserror`.
- Postgres SQL Database Connectivity with `SQLX`.

Workspace containing individual application components (crates).

- ./src/data-collector
- ./src/database

## Quick Start

Data collector will fetch some data, for now this can be tested by running
`cargo run --bin data-collector`
which will build and run only that specific component.

Run a database in a container

```
# Create a directory to store the data file
mkdir -p $HOME/docker/volumes/postgres
# Run a container, should download image if none exists
docker run --rm   \
--name postgres \
-e POSTGRES_PASSWORD=test \
-d -p 5432:5432 \
-v $HOME/docker/volumes/postgres:/var/lib/postgresql/data  postgres
# Create a database to store data in
docker exec -i postgres psql \
-U postgres \
-c "CREATE DATABASE dev WITH ENCODING='UTF8' OWNER=postgres;"
```

Run s3 in a container.

```
docker run -p 9000:9000 \
  -e "MINIO_ACCESS_KEY=AKIAIOSFODNN7EXAMPLE" \
  -e "MINIO_SECRET_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY" \
  minio/minio server /data

```

Run the server.

```
git clone git@github.com:davidmaceachern/rust-web-starter.git
cd /rust-web-starter
cargo run
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 