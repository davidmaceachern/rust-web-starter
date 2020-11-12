FROM rustlang/rust:nightly-slim AS build
WORKDIR /src/rust-web-template
COPY . ./collector
RUN apt-get update && \
    apt-get install pkg-config
RUN cd ./collector && cargo build --release && cd ..
FROM ubuntu:18.04
COPY --from=build /src/rust-web-template/target/release/collector /usr/local/bin/collector
CMD ["usr/local/bin/client"]
