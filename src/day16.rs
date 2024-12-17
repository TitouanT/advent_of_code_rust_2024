use std::collections::BinaryHeap;
const WIDTH: usize = 142;
const HEIGHT: usize = 141;


#[aoc(day16, part1)]
pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut heap: BinaryHeap<(u32,u8,u8,u8)> = BinaryHeap::with_capacity(1000);
    let mut scores = [[[0u32;4];WIDTH];HEIGHT];
    let sl = HEIGHT - 2;
    let sc = 1;
    let el = 1;
    let ec = WIDTH - 3;
    assert_eq!(input[sl*WIDTH + sc], b'S');
    assert_eq!(input[el*WIDTH + ec], b'E');
    heap.push((u32::MAX, sl as u8, sc as u8, 0));
    scores[sl][sc][0] = u32::MAX;
    while let Some((score, cur_l, cur_c, dir)) = heap.pop() {
        let cur_l = cur_l as usize;
        let cur_c = cur_c as usize;
        let dir = dir as usize;
        if score < scores[cur_l][cur_c][dir] {
            // yeah we reached an old state
            continue;
        }
        let maxi = {
            if scores[el][ec][0] == 0 && scores[el][ec][1] == 0 && scores[el][ec][2] == 0 && scores[el][ec][3] == 0 {
                0
            }
            else {
                let first = if scores[el][ec][0] < scores[el][ec][1] { 1 } else { 0 };
                let secnd = if scores[el][ec][2] < scores[el][ec][3] { 3 } else { 2 };
                scores[el][ec][if scores[el][ec][first] < scores[el][ec][secnd] { secnd } else { first }]
            }
        };
        // start elagging only when the end has been reached.
        if maxi > 0 {
            let min_rotations = {
                if cur_l == el  {
                    match dir {
                        0 => 0,
                        2 => 2,
                        _ => 1,
                    }
                }
                else if cur_c == ec {
                    match dir {
                        1 => 0,
                        3 => 2,
                        _ => 1,
                    }
                }
                else {
                    if cur_l > el {
                        if cur_c < ec {
                            match dir {
                                0 | 1 =>  1,
                                _ => 2,
                            }
                        }
                        else {
                            unimplemented!()
                        }
                    }
                    else {
                        unimplemented!();
                    }
                }
            };
            if score - ((cur_l - el) as u32 + (ec - cur_c) as u32 + 1000 * min_rotations) < maxi {
                continue
            }
        }

        let rot_score = score - 1000;
        let walk_score = score - 1;

        let (neigh_dirs, neigh_l, neigh_c) = match dir {
            0 => ([1,3], cur_l, cur_c+1),
            1 => ([2,0], cur_l-1, cur_c),
            2 => ([3,1], cur_l, cur_c-1),
            3 => ([0,2], cur_l+1, cur_c),
            _ => unreachable!(),
        };

        for neigh_dir in neigh_dirs {
            if rot_score > scores[cur_l][cur_c][neigh_dir] {
                scores[cur_l][cur_c][neigh_dir] = rot_score;
                // heap.push((rot_score - expected_cost, rot_score, cur_l as u8, cur_c as u8, neigh_dir as u8));
                heap.push((rot_score, cur_l as u8, cur_c as u8, neigh_dir as u8));
            };
        }
        if input[neigh_l * WIDTH + neigh_c] != b'#' && walk_score > scores[neigh_l][neigh_c][dir] {
            scores[neigh_l][neigh_c][dir] = walk_score;
            // heap.push((walk_score - expected_cost, walk_score, neigh_l as u8, neigh_c as u8, dir as u8));
            heap.push((walk_score, neigh_l as u8, neigh_c as u8, dir as u8));
        }
    }
    let maxi = {
        let first = if scores[el][ec][0] < scores[el][ec][1] { 1 } else { 0 };
        let secnd = if scores[el][ec][2] < scores[el][ec][3] { 3 } else { 2 };
        if scores[el][ec][first] < scores[el][ec][secnd] { secnd } else { first }
    };
    u32::MAX - scores[el][ec][maxi]
}


#[aoc(day16, part2)]
pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut heap: BinaryHeap<(u32,u8,u8,u8)> = BinaryHeap::with_capacity(1000);
    let mut scores = [[[0u32;4];WIDTH];HEIGHT];
    let sl = HEIGHT - 2;
    let sc = 1;
    let el = 1;
    let ec = WIDTH - 3;
    assert_eq!(input[sl*WIDTH + sc], b'S');
    assert_eq!(input[el*WIDTH + ec], b'E');
    scores[sl][sc][0] = u32::MAX - 1000;
    heap.push((scores[sl][sc][0], sl as u8, sc as u8, 0));
    while let Some((score, cur_l, cur_c, dir)) = heap.pop() {
        let cur_l = cur_l as usize;
        let cur_c = cur_c as usize;
        let dir = dir as usize;
        if score < scores[cur_l][cur_c][dir] {
            // yeah we reached an old state
            continue;
        }
        let maxi = {
            if scores[el][ec][0] == 0 && scores[el][ec][1] == 0 && scores[el][ec][2] == 0 && scores[el][ec][3] == 0 {
                0
            }
            else {
                let first = if scores[el][ec][0] < scores[el][ec][1] { 1 } else { 0 };
                let secnd = if scores[el][ec][2] < scores[el][ec][3] { 3 } else { 2 };
                scores[el][ec][if scores[el][ec][first] < scores[el][ec][secnd] { secnd } else { first }]
            }
        };
        // start elagging only when the end has been reached.
        if maxi > 0 {
            let min_rotations = {
                if cur_l == el  {
                    match dir {
                        0 => 0,
                        2 => 2,
                        _ => 1,
                    }
                }
                else if cur_c == ec {
                    match dir {
                        1 => 0,
                        3 => 2,
                        _ => 1,
                    }
                }
                else {
                    if cur_l > el {
                        if cur_c < ec {
                            match dir {
                                0 | 1 =>  1,
                                _ => 2,
                            }
                        }
                        else {
                            unimplemented!()
                        }
                    }
                    else {
                        unimplemented!();
                    }
                }
            };
            if score - ((cur_l - el) as u32 + (ec - cur_c) as u32 + 1000 * min_rotations) < maxi {
                continue
            }
        }

        let rot_score = score - 1000;
        let walk_score = score - 1;

        let (neigh_dirs, neigh_l, neigh_c) = match dir {
            0 => ([1,3], cur_l, cur_c+1),
            1 => ([2,0], cur_l-1, cur_c),
            2 => ([3,1], cur_l, cur_c-1),
            3 => ([0,2], cur_l+1, cur_c),
            _ => unreachable!(),
        };

        for neigh_dir in neigh_dirs {
            if rot_score > scores[cur_l][cur_c][neigh_dir] {
                scores[cur_l][cur_c][neigh_dir] = rot_score;
                // heap.push((rot_score - expected_cost, rot_score, cur_l as u8, cur_c as u8, neigh_dir as u8));
                heap.push((rot_score, cur_l as u8, cur_c as u8, neigh_dir as u8));
            };
        }
        if input[neigh_l * WIDTH + neigh_c] != b'#' && walk_score > scores[neigh_l][neigh_c][dir] {
            scores[neigh_l][neigh_c][dir] = walk_score;
            // heap.push((walk_score - expected_cost, walk_score, neigh_l as u8, neigh_c as u8, dir as u8));
            heap.push((walk_score, neigh_l as u8, neigh_c as u8, dir as u8));
        }
    }
    let maxi = {
        let first = if scores[el][ec][0] < scores[el][ec][1] { 1 } else { 0 };
        let secnd = if scores[el][ec][2] < scores[el][ec][3] { 3 } else { 2 };
        if scores[el][ec][first] < scores[el][ec][secnd] { secnd } else { first }
    };

    let mut stack = [(0u8, 0u8, 0u8);2000];
    let mut stack_size = 0;
    let mut visited = [[[false;5];WIDTH];HEIGHT];
    let mut count = 0;
    for i in 0..4 {
        if scores[el][ec][i] == scores[el][ec][maxi] {
            stack[stack_size] = (el as u8, ec as u8, i as u8);
            visited[el][ec][i] = true;
            stack_size += 1;
        }
    }
    visited[el][ec][4] = true;
    count += 1;

    while stack_size > 0 {
        stack_size -= 1;
        let (cur_l, cur_c, dir) = stack[stack_size];
        let cur_l = cur_l as usize;
        let cur_c = cur_c as usize;
        let dir = dir as usize;
        let score = scores[cur_l][cur_c][dir];
        if cur_l == sl && cur_c == sc {
            continue
        }

        let (neigh_dirs, neigh_l, neigh_c) = match dir {
            0 => ([1,3], cur_l, cur_c-1),
            1 => ([2,0], cur_l+1, cur_c),
            2 => ([3,1], cur_l, cur_c+1),
            3 => ([0,2], cur_l-1, cur_c),
            _ => unreachable!(),
        };

        for neigh_dir in neigh_dirs {
            if visited[cur_l][cur_c][neigh_dir] {
                continue
            }
            if score+1000 == scores[cur_l][cur_c][neigh_dir] {
                stack[stack_size] = (cur_l as u8, cur_c as u8, neigh_dir as u8);
                stack_size += 1;
                visited[cur_l][cur_c][neigh_dir] = true;
            };
        }
        if visited[neigh_l][neigh_c][dir] {
            continue
        }
        if score+1 == scores[neigh_l][neigh_c][dir] {
            stack[stack_size] = (neigh_l as u8, neigh_c as u8, dir as u8);
            stack_size += 1;
            visited[neigh_l][neigh_c][dir] = true;
            if !visited[neigh_l][neigh_c][4] {
                count += 1;
                visited[neigh_l][neigh_c][4] = true;
            }
        }
    }
    count
}

