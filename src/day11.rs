use std::collections::HashMap;

#[inline(always)]
fn div_cut(n: u64) -> Option<u64> {
    if      n < 10 { None }
    else if n < 100 { Some(10) }
    else if n < 1000 { None }
    else if n < 10000 { Some(100) }
    else if n < 100000 { None }
    else if n < 1000000 { Some(1000) }
    else if n < 10000000 { None }
    else if n < 100000000 { Some(10000) }
    else if n < 1000000000 { None }
    else if n < 10000000000 { Some(100000) }
    else if n < 100000000000 { None }
    else if n < 1000000000000 { Some(1000000) }
    else { None }
}
#[inline(always)]
fn add(mapb: &mut HashMap<u64, u64>, key: u64, count: u64) {
    mapb.entry(key).and_modify(|value|*value += count).or_insert(count);
}

#[inline(always)]
fn f(input: &str, n: usize) -> u64 {
    let mut v = 0;
    let mut map1 = HashMap::with_capacity(5000);
    let mut map2 = HashMap::with_capacity(5000);
    for &byte in input.as_bytes() {
        v = if byte < 40 {
            if byte == 32 {
                add(&mut map1, v, 1);
                0
            }
            else {
                v
            }
        }
        else {
            v*10+(byte as u64)-48
        }
    }
    map1.entry(v).and_modify(|v| *v+=1).or_insert(1);
    let (mut ping, mut pong) = (&mut map1, &mut map2);
    for _ in 0..n {
        for (stone, count) in ping.drain() {
            if stone == 0 {
                add(&mut pong, 1, count);
            }
            else if let Some(div) = div_cut(stone) {
                add(&mut pong, stone/div, count);
                add(&mut pong, stone%div, count);
            }
            else {
                add(&mut pong, stone*2024, count);
            }
        }
        (ping, pong) = (pong, ping)
    }
    ping.values().sum()
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u64 {f(input, 25)}
#[aoc(day11, part2)]
pub fn part2(input: &str) -> u64 {f(input, 75)}
