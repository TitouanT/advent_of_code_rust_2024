use std::collections::HashMap;

fn parse_int(input: &str) -> i32 {
    input.parse::<i32>().expect("Valid input")
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left : Vec<i32> = vec![];
    let mut right : Vec<i32> = vec![];
    let re = regex::Regex::new(r"([[:digit:]]+) +([[:digit:]]+)").expect("Regex fu");
    for (_, [i_left, i_right]) in re.captures_iter(input).map(|cap| cap.extract()) {
        left.push(parse_int(i_left));
        right.push(parse_int(i_right));
    }
    (left, right)
}

fn counter<T: Eq + std::hash::Hash, U: IntoIterator<Item=T>> (l:U) -> HashMap<T, i32> {
    let mut c : HashMap<T, i32> = HashMap::new();
    for k in l {
        c.entry(k).and_modify(|v|*v+=1).or_insert(1);
    }
    c
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    let mut t = 0;
    for (a, b) in left.iter().zip(right.iter()) {
        t += (a-b).abs()
    }
    t
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let (lft, rgt) = parse_input(input);
    let rgt_counter = counter(rgt);
    let mut t = 0;
    for k in &lft {
        t += k*rgt_counter.get(k).unwrap_or(&0);
    }
    t
}
