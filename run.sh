#!/bin/bash

NIGHTLY=nightly-2021-01-20
TARGET_NAME=tls-enclave
TARGET_DIR=ra-enclave/target/x86_64-fortanix-unknown-sgx/debug/examples
TARGET=$TARGET_DIR/$TARGET_NAME.sgxs

# Run enclave with the default runner
ftxsgx-runner --signature coresident $TARGET &

# Run client
(cd ra-client && cargo +$NIGHTLY run -Zfeatures=itarget --example tls-client --features verbose) &

# Run SP
(cd ra-sp && cargo +$NIGHTLY run -Zfeatures=itarget --example tls-sp --features "verbose")
