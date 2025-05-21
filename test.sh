#!/bin/bash
set -e

for file in $(find . -type f)
do
  echo catting $file

  cargo run -- $file
done
