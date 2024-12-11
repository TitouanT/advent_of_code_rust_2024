#!/bin/bash
YEAR=2024
# maybe query that from Cargo.toml
REPO=advent_of_code_rust_2024
for dayi in $(seq 1 $1)
do
    day="day${dayi}"
    [ ! -f "benches/${day}.rs" ] && sed "s:__DAY__:${day}:g ; s:__YEAR__:${YEAR}:g ; s:__REPO__:${REPO}:g" templates/day.rs > "benches/${day}.rs"
    grep -q "${day}" Cargo.toml || sed "s:__DAY__:${day}:g" templates/cargo_bench.toml >> Cargo.toml
    ./get_input_output.sh "${dayi}"
done

./hide_input.sh
