#!/bin/bash

NIGHTLY=nightly-2021-02-20
TARGET_NAME=tls-enclave
TARGET_DIR=ra-enclave/target/x86_64-fortanix-unknown-sgx/debug/examples
TARGET=$TARGET_DIR/$TARGET_NAME
TARGET_SGX=$TARGET_DIR/$TARGET_NAME.sgxs
TARGET_SIG=$TARGET_DIR/$TARGET_NAME.sig
KEY=ra-enclave/examples/data/vendor-keys/private_key.pem

rustup target add x86_64-fortanix-unknown-sgx --toolchain $NIGHTLY
#cargo +$NIGHTLY install fortanix-sgx-tools sgxs-tools

cp ~/sgx_keys/*  ra-sp/examples/data


# Build and sign enclave
(cd ra-enclave && cargo +$NIGHTLY build --target x86_64-fortanix-unknown-sgx -Zfeatures=itarget --example tls-enclave --features example) && \
ftxsgx-elf2sgxs $TARGET --heap-size 0x2000000 --stack-size 0x20000 --threads 8 \
    --debug --output $TARGET_SGX && \
sgxs-sign --key $KEY $TARGET_SGX $TARGET_SIG -d --xfrm 7/0 --isvprodid 0 --isvsvn 0

# Build client
(cd ra-client && cargo +$NIGHTLY build -Zfeatures=itarget --example tls-client --features verbose)

# Build SP
(cd ra-sp && cargo +$NIGHTLY build -Zfeatures=itarget --example tls-sp --features "verbose")

