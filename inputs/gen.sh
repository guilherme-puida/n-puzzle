#!/usr/bin/env sh

amount=$1
lines=$2
cols=$3

for i in $(seq 0 $amount); do
  ./generator $lines $cols 1000 > "${lines}x$cols-$i.input"
done
