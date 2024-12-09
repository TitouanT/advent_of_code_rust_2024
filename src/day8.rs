const N: usize = 50;
const MAX_ANTENNAS: usize = 8;
const MAX_FREQUENCY: usize = 128;
use std::ops::Sub;

#[derive(Clone, Copy, Debug)]
struct Position {
    line: usize,
    col: usize,
}

#[derive(Clone, Copy)]
struct Direction {
    dl: i32,
    dc: i32,
}

fn in_bound(n: i32) -> bool {
    0 <= n && n < N as i32
}

// impl Direction {
//     fn rotate(&mut self) {
//         (self.dl, self.dc) = (self.dc, -self.dl);
//     }
// }


impl Position {
    fn add_direction(&self, direction: &Direction) -> Option<Self> {
        let nl = direction.dl + self.line as i32;
        let nc = direction.dc + self.col as i32;

        if in_bound(nl) && in_bound(nc) {
            Some(Position {
                line: nl as usize,
                col: nc as usize,
            })
        }
        else {
            None
        }
    }
}

impl Sub for Position {
    type Output = Direction;
    fn sub(self, other: Position) -> Self::Output {
        Direction {
            dl: self.line as i32 - other.line as i32,
            dc: self.col as i32 - other.col as i32
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Antennas {
    size: usize,
    members: [Position;MAX_ANTENNAS]
}

impl Antennas {
    fn add_member(&mut self, antenna: Position) {
        self.members[self.size] = antenna;
        self.size += 1;
    }
}

fn parse_input(input: &str) -> [Antennas;MAX_FREQUENCY] {
    let mut antennas = [Antennas {
        size: 0,
        members: [
            Position {
                line: 0,
                col: 0,
            };
            MAX_ANTENNAS
        ]
    };MAX_FREQUENCY];
    for (i, line) in input.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        for (j, cell) in line.chars().enumerate() {
            if cell == '.' {
                continue
            }
            antennas[cell as usize].add_member(Position{line:i, col:j})
        }
    }
    antennas
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> u32 {
    let antennas = parse_input(input);
    let mut is_free = [[true; N]; N];
    // println!("{:?}", antennas);
    let mut count = 0;
    for antenna in antennas.iter() {
        if antenna.size < 2 {
            continue
        }
        let n = antenna.size;
        for (i, &a) in antenna.members[..n].iter().enumerate() {
            for &b in antenna.members[i+1..n].iter() {
                if let Some(next) = a.add_direction(&(a-b)) {
                    if is_free[next.line][next.col] {
                        count += 1;
                        is_free[next.line][next.col] = false;
                    }
                }
                if let Some(next) = b.add_direction(&(b-a)) {
                    if is_free[next.line][next.col] {
                        count += 1;
                        is_free[next.line][next.col] = false;
                    }
                }
            }
        }
    }
    count
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> u32 {
    let antennas = parse_input(input);
    let mut is_free = [[true; N]; N];
    // println!("{:?}", antennas);
    let mut count = 0;
    for antenna in antennas.iter() {
        if antenna.size < 2 {
            continue
        }
        let n = antenna.size;
        for (i, &a) in antenna.members[..n].iter().enumerate() {
            for &b in antenna.members[i+1..n].iter() {
                let mut current = b;
                while let Some(next) = current.add_direction(&(a-b)) {
                    if is_free[next.line][next.col] {
                        count += 1;
                        is_free[next.line][next.col] = false;
                    }
                    current = next;
                }
                let mut current = a;
                while let Some(next) = current.add_direction(&(b-a)) {
                    if is_free[next.line][next.col] {
                        count += 1;
                        is_free[next.line][next.col] = false;
                    }
                    current = next;
                }
            }
        }
    }
    count
}
