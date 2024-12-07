const N: usize = 130;

#[derive(Clone, Copy)]
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

impl Direction {
    fn rotate(&mut self) {
        (self.dl, self.dc) = (self.dc, -self.dl);
    }
    fn as_bit(&self) -> u8 {
        if self.dl == -1 { 0 }
        else if self.dl == 1 { 1 }
        else if self.dc == 1 { 2 }
        else { 4 }
    }
}


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

fn parse_input(input: &str) -> ([[bool; N]; N], Position) {
    let mut map = [[false; N]; N];
    let mut pos = Position {line: 0, col: 0};
    for (i, line) in input.split("\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        for (j, cell) in line.chars().enumerate() {
            if cell == '#' {
                map[i][j] = true;
            }
            if cell == '^' {
                pos.line = i;
                pos.col = j;
            }
        }
    }
    (map, pos)
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> u32 {
    let (map, mut pos) = parse_input(input);
    let mut visited = [[false; N]; N];
    let mut direction = Direction { dl: -1, dc: 0 };
    visited[pos.line][pos.col] = true;
    let mut count = 1;
    while let Some(next_pos) = pos.add_direction(&direction) {
        if map[next_pos.line][next_pos.col] {
            direction.rotate();
        }
        else {
            pos = next_pos;
            if !visited[pos.line][pos.col] {
                visited[pos.line][pos.col] = true;
                count += 1;
            }
        }
    }
    count
}

fn is_stuck(map: &[[bool; N]; N], pos: &Position, direction: &Direction) -> bool {
    let mut pos = Position{line:pos.line, col:pos.col};
    let mut direction = Direction{dl:direction.dl, dc:direction.dc};
    let mut visited = [[0u8; N]; N];
    visited[pos.line][pos.col] |= direction.as_bit();
    while let Some(next_pos) = pos.add_direction(&direction) {
        if map[next_pos.line][next_pos.col] {
            direction.rotate();
        }
        else {
            pos = next_pos;
            let b_direction = direction.as_bit();
            if visited[pos.line][pos.col] & b_direction == 0 {
                visited[pos.line][pos.col] |= b_direction;
            }
            else {
                return true
            }
        }
    }
    false
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let (mut map, mut pos) = parse_input(input);
    let mut visited = [[false; N]; N];
    let mut direction = Direction { dl: -1, dc: 0 };
    let mut count = 0;
    visited[pos.line][pos.col] = true;
    while let Some(next_pos) = pos.add_direction(&direction) {
        if map[next_pos.line][next_pos.col] {
            direction.rotate();
        }
        else {
            if !visited[next_pos.line][next_pos.col] {
                map[next_pos.line][next_pos.col] = true;
                if is_stuck(&map, &pos, &direction) {
                    count+=1
                }
                map[next_pos.line][next_pos.col] = false;
                visited[next_pos.line][next_pos.col] = true;
            }
            pos = next_pos;
        }
    }
    count
}
