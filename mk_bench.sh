#!/bin/bash
for day in $(seq 1 $1)
do
    sed "s:__DAY__:day${day}:g ; s:__YEAR__:2024:g ; s:__REPO__:advent_of_code_rust_2024:g" templates/day.rs > benches/day${day}.rs
done
