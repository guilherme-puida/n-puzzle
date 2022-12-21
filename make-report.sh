#!/usr/bin/env bash

sources="3x3-0 3x3-5 3x3-10 3x3-15 3x3-20 4x4-0 4x4-5 4x4-10 4x4-15 4x4-20 5x5-0 5x5-1 5x5-2 5x5-3 5x5-5 5x5-9 5x5-10 5x5-15 5x5-20"

cargo build --release
rm report

for i in $sources; do
  echo "solving $i"
  time ./target/release/solve < ./inputs/$i.input
  echo
done | tee report