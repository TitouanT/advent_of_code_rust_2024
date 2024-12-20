use std::collections::BinaryHeap;

const WIDTH: usize = 71;
const HEIGHT: usize = 71;
const NWALL: u16 = 3450;

const WIDTH_I8: i8 = WIDTH as i8;
const HEIGHT_I8: i8 = HEIGHT as i8;
const NWALL_USIZE: usize = NWALL as usize;

fn read_input(input: &str) -> [[bool;WIDTH];HEIGHT] {
    let input = input.as_bytes();
    let mut is_wall = [[false;WIDTH];HEIGHT];
    let mut index = 0;
    for _ in 1..=1024 {
        let mut line = 0;
        while input[index] != b',' {
            line = line * 10 + input[index] - b'0';
            index += 1;
        }
        index += 1;
        let mut col = 0;
        while input[index] != b'\n' {
            col = col * 10 + input[index] - b'0';
            index += 1;
        }
        index += 1;
        is_wall[line as usize][col as usize] = true;
    }
    is_wall
}

fn read_input_p2(input: &str) -> ([[u16;WIDTH];HEIGHT], [&str;NWALL_USIZE]) {
    let raw_input = input;
    let input = input.as_bytes();
    let mut is_wall = [[NWALL;WIDTH];HEIGHT];
    let mut index = 0;
    let mut raw_lines = [&raw_input[..1];NWALL_USIZE];
    for (i, rawi) in raw_lines.iter_mut().enumerate() {
        let start_index = index;
        let mut line = 0;
        while input[index] != b',' {
            line = line * 10 + input[index] - b'0';
            index += 1;
        }
        index += 1;
        let mut col = 0;
        while index < input.len() && input[index] != b'\n' {
            col = col * 10 + input[index] - b'0';
            index += 1;
        }
        let end_index = index;
        *rawi = &raw_input[start_index..end_index];
        index += 1;
        is_wall[line as usize][col as usize] = i as u16;
    }
    (is_wall, raw_lines)
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u32 {
    let mut is_wall = read_input(input);
    let mut level      = &mut [(0,0);2*WIDTH];
    let mut next_level = &mut [(0,0);2*WIDTH];
    let mut n_level = 1;

    // let mut ref_level = &mut level;
    // let mut ref_next_level = &mut next_level;
    let mut current_level = 0;
    loop {
        current_level += 1;
        let mut next_level_size = 0;
        for (curl, curc) in level.iter().take(n_level) {
            for (next_l, next_c) in  &[
                (curl+1, *curc),
                (curl-1, *curc),
                (*curl, curc+1),
                (*curl, curc-1),
            ] {
                let next_l = *next_l;
                let next_c = *next_c;

                if !(0..HEIGHT_I8).contains(&next_l) || !(0..WIDTH_I8).contains(&next_c) {
                    continue;
                }
                if next_l == HEIGHT_I8 - 1 && next_c == WIDTH_I8 - 1 {
                    return current_level;
                }
                if is_wall[next_l as usize][next_c as usize] {
                    continue;
                }
                is_wall[next_l as usize][next_c as usize] = true;
                next_level[next_level_size] = (next_l, next_c);
                next_level_size += 1;
            }
        }
        // (ref_level, ref_next_level) = (ref_next_level, ref_level);
        (level, next_level) = (next_level, level);
        n_level = next_level_size;
    }
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    let (wall_score, raw_pos) = read_input_p2(input);
    let mut path_score = [[0;WIDTH];HEIGHT];
    let mut queue: BinaryHeap<(u16, u8, u8)> = BinaryHeap::new();
    queue.push((NWALL, 0, 0));
    path_score[0][0] = NWALL;
    while let Some((current_score, curl, curc)) = queue.pop() {
        for (dl, dc) in &[(0,1), (1,0), (-1, 0), (0, -1)] {
            let neigh_l = dl + curl as i8;
            let neigh_c = dc + curc as i8;
            if !(0..HEIGHT_I8).contains(&neigh_l) || !(0..WIDTH_I8).contains(&neigh_c) {
                continue;
            }
            let neigh_l = neigh_l as usize;
            let neigh_c = neigh_c as usize;


            // to continue on this path we need to lower the score
            // the good news for that node is that if it was already visited, it is now likely
            // the best path going through.
            let neigh_score = {
                if wall_score[neigh_l][neigh_c] <= current_score {
                    wall_score[neigh_l][neigh_c]
                }
                else {
                    current_score
                }
            };

            if neigh_score > path_score[neigh_l][neigh_c] {
                // the score of a path, can only decrease, and we always make the highest
                // scored path advance first so we cannot be in a situation where a previous
                // path would have written a lower score there.
                //
                // the only way this can happen, is if the path was never visited.
                assert_eq!(path_score[neigh_l][neigh_c], 0);
                if neigh_l == HEIGHT - 1 && neigh_c == WIDTH - 1 {
                    return raw_pos[neigh_score as usize].to_string();
                }
                path_score[neigh_l][neigh_c] = neigh_score;
                queue.push((neigh_score, neigh_l as u8, neigh_c as u8));
            }
            else if current_score == path_score[neigh_l][neigh_c] {
                // this is possible, we have nothing to do, its already taken care of.
                continue;
            }
            else {
                // our score is lower than what a previous path did ?
                // creep;
                continue;
            }
        }
    }
    raw_pos[0].to_string()
}

