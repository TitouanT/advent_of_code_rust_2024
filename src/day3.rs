
fn parse_int(input: &str) -> i32 {
    input.parse::<i32>().expect("Valid input")
}

// fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
//     let mut left : Vec<i32> = vec![];
//     let mut right : Vec<i32> = vec![];
//     let re = regex::Regex::new(r"([[:digit:]]+) +([[:digit:]]+)").expect("Regex fu");
//     for (_, [i_left, i_right]) in re.captures_iter(input).map(|cap| cap.extract()) {
//         left.push(parse_int(i_left));
//         right.push(parse_int(i_right));
//     }
//     (left, right)
// }


#[aoc(day3, part1)]
pub fn part1(input: &str) -> i32 {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").expect("Regex fu");
    // let regexes = regex::RegexSet::new(&[

    // ]);
    let mut t = 0;
    for (_, [i_left, i_right]) in re.captures_iter(input).map(|cap| cap.extract()) {
        t += parse_int(i_left) * parse_int(i_right);
    }
    t
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i32 {
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("Regex fu");
    let mut t = 0;
    let mut doit = 1;
    for cap in re.captures_iter(input) {
        let matched_string = cap.get(0).map_or("", |m| m.as_str());
        match matched_string {
            "do()" => doit = 1,
            "don't()" => doit = 0,
            _ => {
                let a = cap.get(1).map_or("", |m| m.as_str());
                let b = cap.get(2).map_or("", |m| m.as_str());
                t += parse_int(a) * parse_int(b) * doit;
            }
        }
    }
    t
}
