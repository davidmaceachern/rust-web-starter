FROM rustlang/rust:nightly-slim AS build
WORKDIR /src/rust-web-template
COPY . ./collector
RUN apt-get update -y && \
    apt-get install -y pkg-config \
    libssl-dev
RUN cd ./collector && cargo build --release && cd ..
FROM ubuntu:18.04
COPY --from=build /src/rust-web-template/collector/target/release/collector /usr/local/bin/collector
CMD ["usr/local/bin/client"]
