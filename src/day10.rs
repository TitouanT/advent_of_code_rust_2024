// use std::collections::HashSet;
const N: usize = 55;

// fn all_uniq_9s(grid:&[[u8;N];N], l: usize, c: usize, h: &mut HashSet<u16>) {
fn all_uniq_9s(grid:&[[u8;N];N], l: usize, c: usize) -> u32 {
    // let target = grid[l][c];
    // if target == b'9' {
    //     h.insert((l*N + c) as u16);
    //     return
    // }
    // if l > 0 && grid[l-1][c] == target+1 {
    //     all_uniq_9s(grid, l-1, c, h);
    // }
    // if l < N-1 && grid[l+1][c] == target+1 {
    //     all_uniq_9s(grid, l+1, c, h);
    // }
    // if c > 0 && grid[l][c-1] == target+1 {
    //     all_uniq_9s(grid, l, c-1, h);
    // }
    // if c < N-1 && grid[l][c+1] == target+1 {
    //     all_uniq_9s(grid, l, c+1, h);
    // }

    let mut to_visit = [[true;N];N];
    let mut queue = [(0u8,0u8);N*N];
    queue[0] = (l as u8, c as u8);
    let mut start = 0;
    let mut end = 1;
    let mut count = 0;
    to_visit[l][c] = false;
    while start < end {
        let (lu8, cu8) = queue[start];
        let l = lu8 as usize;
        let c = cu8 as usize;
        start += 1;
        let target = grid[l][c];
        let count_update = if target == b'8' { 1 } else { 0 };
        if l > 0 && grid[l-1][c] == target+1 && to_visit[l-1][c] {
            to_visit[l-1][c] = false;
            queue[end]=(lu8-1,cu8);
            end+=1;
            count += count_update;
        }
        if l < N-1 && grid[l+1][c] == target+1 && to_visit[l+1][c]{
            to_visit[l+1][c] = false;
            queue[end]=(lu8+1,cu8);
            end+=1;
            count += count_update;
        }
        if c > 0 && grid[l][c-1] == target+1 && to_visit[l][c-1]{
            to_visit[l][c-1] = false;
            queue[end]=(lu8,cu8-1);
            end+=1;
            count += count_update;
        }
        if c < N-1 && grid[l][c+1] == target+1 && to_visit[l][c+1]{
            to_visit[l][c+1] = false;
            queue[end]=(lu8,cu8+1);
            end+=1;
            count += count_update;
        }
    }
    count
}

fn all_9s(grid:&[[u8;N];N], l: usize, c: usize) -> u32 {
    let target = grid[l][c];
    if target == b'9' {
        return 1;
    }
    (
        if l >  0  && grid[l-1][c  ] == target+1 { all_9s(grid, l-1, c  ) } else { 0 }
        +
        if l < N-1 && grid[l+1][c  ] == target+1 { all_9s(grid, l+1, c  ) } else { 0 }
        +
        if c >  0  && grid[l  ][c-1] == target+1 { all_9s(grid, l  , c-1) } else { 0 }
        +
        if c < N-1 && grid[l  ][c+1] == target+1 { all_9s(grid, l  , c+1) } else { 0 }
    )
}


#[aoc(day10, part1)]
pub fn part1(input: &str) -> u32 {
    let input = &input.as_bytes()[..(N+1)*(N+1)-N-2];
    let mut grid = [[0u8;N];N];
    for l in 0..N {
        grid[l] = input[l*(N+1)..l*(N+1)+N].try_into().expect("copyyyyy");
    }
    let mut count = 0;
    for l in 0..N {
        for c in 0..N {
            if grid[l][c] == b'0' {
                // let mut h = HashSet::new();
                count += all_uniq_9s(&grid, l, c);
                // count += h.len();
            }
        }
    }
    count
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> u32 {
    let input = &input.as_bytes()[..(N+1)*(N+1)-N-2];
    let mut grid = [[0u8;N];N];
    for l in 0..N {
        grid[l] = input[l*(N+1)..l*(N+1)+N].try_into().expect("copyyyyy");
    }
    let mut count = 0;
    for l in 0..N {
        for c in 0..N {
            if grid[l][c] == b'0' {
                count += all_9s(&grid, l, c);
            }
        }
    }
    count
}
