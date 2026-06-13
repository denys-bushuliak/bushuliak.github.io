#!/bin/bash

# Check if cargo-llvm-cov is installed
if ! command -v cargo-llvm-cov &> /dev/null
then
    echo "cargo-llvm-cov could not be found. Installing it..."
    cargo install cargo-llvm-cov
fi

# Run coverage and generate HTML report
echo "Running coverage..."
cargo llvm-cov --html --open
