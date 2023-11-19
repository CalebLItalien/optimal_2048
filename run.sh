#!/bin/bash

# run.sh

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <iterations> <goal>"
    exit 1
fi

cargo build --release

./target/release/optimal_2048 "$1" "$2"
