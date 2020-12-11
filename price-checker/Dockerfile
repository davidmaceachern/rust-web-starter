FROM rustlang/rust:nightly-slim AS build
WORKDIR /src/rust-web-template
COPY . ./price-checker
RUN apt-get update -y && \
    apt-get install -y pkg-config \
    libssl-dev
RUN cd ./price-checker && cargo build --release && cd ..
FROM ubuntu:18.04
COPY --from=build /src/rust-web-template/price-checker/target/release/price-checker /usr/local/bin/price-checker
CMD ["usr/local/bin/price-checker"]
