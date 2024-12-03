fn parse_int(input: &str) -> i32 {
    input.parse::<i32>().expect("Valid input")
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut lines = vec![];
    for l in input.split("\n") {
        //println!("{:?}", l);
        if l.len() > 1 {
            let line_vec : Vec<i32> = l.split(" ").map(parse_int).collect();
            lines.push(line_vec);
        }
        //println!("{:?}", line_vec);
    }
    lines
}

fn is_safe(line: &[i32]) -> bool {
    let diffs: Vec<i32> = line.iter().zip(line.iter().skip(1)).map(|(a,b)| a-b).collect();

    let min: &i32 = diffs.iter().min().expect("a minimum value");
    let max: &i32 = diffs.iter().max().expect("a maximum value");

    *min >= -3 && *max <= 3 && (*max < 0 || *min > 0)
}


#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    let mut t = 0;
    for line in parse_input(input) {
        t += is_safe(&line) as i32;
    }
    t
}
#[aoc(day2, part2)]
pub fn part2(input: &str) -> i32 {
    let mut t = 0;
    for line in parse_input(input) {
        let mut can_be_safe = false;
        for i in 0..line.len() {
            let left = &line[..i];
            let right = &line[i+1..];
            can_be_safe |= is_safe(&[left, right].concat())

        }
        t += can_be_safe as i32;
    }
    t
}
