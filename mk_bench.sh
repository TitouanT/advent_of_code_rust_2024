#!/bin/bash
YEAR=2024
# maybe query that from Cargo.toml
REPO=advent_of_code_rust_2024
need_hiding=0
for dayi in $(seq 1 $1)
do
    day="day${dayi}"
    sed "s:__DAY__:${day}:g ; s:__YEAR__:${YEAR}:g ; s:__REPO__:${REPO}:g" templates/day.rs > "benches/${day}.rs"
    grep -q "${day}" Cargo.toml || sed "s:__DAY__:${day}:g" templates/cargo_bench.toml >> Cargo.toml

    [ ! -f input/${YEAR}/${day}.txt ] && cargo aoc input --day "${dayi}"
    [ ! -f input/${YEAR}/${day}.txt.gpg ] && need_hiding=1
done

if [ "${need_hiding}" -eq 1 ]
then
    ./hide_input.sh
fi
