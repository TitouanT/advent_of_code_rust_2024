# AOC 2024 in rust
[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/TitouanT/advent_of_code_rust_2024)

My participation to [aoc 2024](https://adventofcode.com/2024) in Rust.

And also trying to meet the 36h rust leaderboard deadline from [codspeed advent](https://codspeed.io/advent).


## Setup description

For the inputs you need to put a secret passphrase in `.passphrase`
and add it to your variables as `AOC_INPUT_PASSPHRASE`
It is used to encrypt your inputs and outputs for each day with gpg.

You will also need to create a `.cookie` file with a content resembling:
```bash
cookie='session=YOUR_AOC_COOKIE_HERE'
```

To add an input for a day, do:
```
./get_input_output.sh <day number> # get
./hide_input.sh
```
Then in the runner, the inputs will be made available by `show_input.sh`

To add a benchmark simply do
```
./mk_bench.sh [NUMBER_OF_DAY]
```
For exemple `./mk_bench.sh 4` will create bench files for days 1, 2, 3 and 4, download inputs if needed and hide them if needed


## TODO's

```
day12: !!
day19: Do a prefix tree
```
