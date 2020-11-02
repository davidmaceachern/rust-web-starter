FROM rustlang/rust:nightly-slim AS build
WORKDIR /src/rust-web-template
COPY . .
RUN cd ./client && cargo build --release && cd ..
FROM ubuntu:18.04
COPY --from=build /src/rust-web-template/target/release/client /usr/local/bin/client
CMD ["usr/local/bin/client"]

# docker build -t rust-web-template .
