#[derive(Debug)]
struct Member {
    value: u64,
    shift: u64,
}

fn parse_int(input: &str) -> u64 {
    input.parse::<u64>().expect("Valid input")
}

#[derive(Debug)]
struct Equation {
    solution: u64,
    members: Vec<Member>,
}

const SHIFTS : [u64; 5] = [1, 10, 100, 1000, 10000];
fn parse_input(input: &str) -> Vec<Equation> {
    let mut ans = vec![];
    for line in input.split('\n') {
        if let [solution, members] = line.split(": ").collect::<Vec<&str>>()[..] {
            ans.push(Equation {
                solution: parse_int(solution),
                members: members.split(" ").map(
                    |v| {
                        Member {
                            value: parse_int(v),
                            shift: SHIFTS[v.len()],
                        }
                    }
                ).collect()
            })
        }
    }
    ans
}

fn is_solvable_p1_helper(eq: &Equation, n: usize, acc: u64) -> bool {
    if n >= eq.members.len() {
        return acc == eq.solution;
    }
    if acc > eq.solution {
        return false
    }

    let head = &eq.members[n];
    is_solvable_p1_helper(eq, n+1, acc+head.value)
    ||
    is_solvable_p1_helper(eq, n+1, acc*head.value)
}

fn is_solvable_p1(eq: &Equation) -> bool {
    is_solvable_p1_helper(eq, 1, eq.members[0].value)
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u64 {
    let equations = parse_input(input);
    equations.into_iter().filter(is_solvable_p1).map(|eq| eq.solution).sum()
}

fn is_solvable_p2_helper(eq: &Equation, n: usize, acc: u64) -> bool {
    if n >= eq.members.len() {
        return acc == eq.solution;
    }
    if acc > eq.solution {
        return false
    }

    let head = &eq.members[n];
    is_solvable_p2_helper(eq, n+1, acc+head.value)
    ||
    is_solvable_p2_helper(eq, n+1, acc*head.value)
    ||
    is_solvable_p2_helper(eq, n+1, acc*head.shift + head.value)
}

fn is_solvable_p2(eq: &Equation) -> bool {
    is_solvable_p2_helper(eq, 1, eq.members[0].value)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u64 {
    let equations = parse_input(input);
    equations.into_iter().filter(is_solvable_p2).map(|eq| eq.solution).sum()
}

