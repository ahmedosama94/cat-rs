#!/bin/bash
set -e

list=$(find . -type f)
cargo build --release

for file in $list
do
  ./target/release/cat $file
done
