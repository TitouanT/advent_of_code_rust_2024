#!/bin/bash
for day in $(seq 1 $1)
do
    day="day${day}"
    sed "s:__DAY__:${day}:g ; s:__YEAR__:2024:g ; s:__REPO__:advent_of_code_rust_2024:g" templates/day.rs > "benches/${day}.rs"
    grep -q "${day}" Cargo.toml || sed "s:__DAY__:${day}:g" templates/cargo_bench.toml >> Cargo.toml
done
