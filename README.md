# AOC 2024 in rust

My participation to [aoc 2024](https://adventofcode.com/2024) in Rust.

And also trying to meet the 36h rust leaderboard deadline from [codspeed advent](https://codspeed.io/advent).


## Setup description

For the inputs you need to put a secret passphrase in `.passphrase`
and add it to your variables as `AOC_INPUT_PASSPHRASE`

To add an input for a day, do:
```
cargo aoc input [--day <DAY>]
./hide_input.sh
```
Then in the runner, the inputs will be made available by `show_input.sh`

To add a benchmark simply do
```
./mk_bench.sh [NUMBER_OF_DAY]
```
For exemple `./mk_bench.sh 4`

