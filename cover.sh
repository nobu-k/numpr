#!/bin/bash

mkdir -p target
find target -name 'numpr*.gc*' | xargs rm

export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"

cargo +nightly build --workspace
cargo +nightly test --workspace
grcov ./target/debug/ -s numpr -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
