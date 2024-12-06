const N: usize = 100;
use std::cmp;

fn parse_int(s: &str) -> usize {
    s.parse::<usize>().expect("Valid input")
}

fn parse_rules(rules: &[&str]) -> [[cmp::Ordering; N]; N]{
    let mut ans = [[cmp::Ordering::Equal; N]; N];
    for rule in rules {
        let a = parse_int(&rule[..2]);
        let b = parse_int(&rule[3..]);
        ans[a][b] = cmp::Ordering::Less;
        ans[b][a] = cmp::Ordering::Greater;
    }
    ans
}
fn parse_jobs(jobs: &[&str]) -> Vec<Vec<usize>>{
    let mut ans: Vec<Vec<usize>> = vec![];
    for job in jobs {
        let mut pages: Vec<usize> = vec![];
        for i in (0..job.len()).step_by(3) {
            pages.push(parse_int(&job[i..i+2]));
        }
        ans.push(pages);
    }
    ans
}
fn parse_input(input: &str) -> ([[cmp::Ordering; N]; N], Vec<Vec<usize>>) {
    let mut rules = vec![];
    let mut jobs = vec![];
    if let [raw_rules, raw_jobs] = input.split("\n\n").collect::<Vec<_>>()[..] {
        for line in raw_rules.split("\n") {
            rules.push(line);
        }
        for line in raw_jobs.split("\n") {
            if line.is_empty() {
                continue;
            }
            jobs.push(line);
        }
    }
    (parse_rules(&rules), parse_jobs(&jobs))
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let (order, jobs) = parse_input(input);
    jobs
        .iter()
        .filter(|&job| job.is_sorted_by(|&a, &b| order[a][b] == cmp::Ordering::Less))
        .map(|job| job[job.len()/2])
        .sum()
}
#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let (order, mut jobs) = parse_input(input);
    jobs
        .iter_mut()
        .filter(|job| !job.is_sorted_by(|&a, &b| order[a][b] == cmp::Ordering::Less))
        .map(|job| {let n = job.len()/2; *(job.select_nth_unstable_by(n, |&a, &b| order[a][b]).1) })
        .sum()
}
