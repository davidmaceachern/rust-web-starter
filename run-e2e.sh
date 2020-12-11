#!/bin/bash

# Run infrastructure
# in separate terminal run `bash ./run-s3.sh`
# then run this script `bash ./run-e2e.sh`
# Compile collector
cd ./collector && cargo build --release && cd ..
# Compile price-checker
cd ./price-checker && cargo build --release && cd ..
# Run collector three times
./target/release/collector
# ./target/release/collector
# ./target/release/collector
# Run price-checker
./target/release/price-checker