const GRID_WIDTH: usize = 50;
const GRID_HEIGHT: usize = 50;
const GRID_END: usize = (GRID_WIDTH+1)*GRID_HEIGHT;
const N_CMD_LINE: usize = 20;
const N_CMD_PER_LINE: usize = 1000;

// use std::{thread, time};

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut grid: [[u8;GRID_WIDTH];GRID_HEIGHT] = [[0u8;GRID_WIDTH];GRID_HEIGHT];
    let cmd = &input[GRID_END+1..];
    let mut pl = 0;
    let mut pc = 0;
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            let letter = input[i*GRID_WIDTH+i + j];
            if letter == b'@' {
                pl = i;
                pc = j;
                grid[i][j] = b'.';
            }
            else {
                grid[i][j] = letter;
            }
        }
    }
    let mut index = 0;
    for _ in 0..N_CMD_LINE {
        for _ in 0..N_CMD_PER_LINE {
            let (dl, dc) = match cmd[index] {
                b'^' => (-1isize, 0isize),
                b'v' => ( 1isize, 0isize),
                b'>' => ( 0isize, 1isize),
                b'<' => ( 0isize,-1isize),
                _ => unreachable!(),
            };
            index += 1;

            let new_l = (pl as isize + dl) as usize;
            let new_c = (pc as isize + dc) as usize;
            match grid[new_l][new_c] {
                b'O' => {
                    let mut free_l = new_l;
                    let mut free_c = new_c;
                    while grid[free_l][free_c] == b'O' {
                        free_l = (free_l as isize + dl) as usize;
                        free_c = (free_c as isize + dc) as usize;
                    }

                    if grid[free_l][free_c] == b'#' {
                        continue;
                    }

                    grid[free_l][free_c] = b'O';
                    grid[new_l][new_c] = b'.';
                },
                b'#' => {
                    continue;
                },
                _ => (),
            }
            pl = new_l;
            pc = new_c;
        }
        index += 1;
    }
    assert_eq!(index, N_CMD_LINE * (N_CMD_PER_LINE+1));
    let mut total = 0;
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            if grid[i][j] == b'O'  {
                total += i*100 + j;
            }
        }
    }
    total
}
// update model for total
    // let mut total = 0;
    // for i in 0..GRID_HEIGHT {
    //     for j in 0..GRID_WIDTH {
    //         match grid[i*GRID_WIDTH+i + j] {
    //             b'@' => {
    //                 pl = i;
    //                 pc = j;
    //             },
    //             b'O' => {
    //                 total += 100 * i + j;
    //             },
    //             _ => (),
    //         }
    //     }
    // }
    // let mut total = total as isize;
    // let n_cell = (free_l as isize - new_l as isize).abs() + (free_c as isize - new_c as isize).abs();
    // let droped: isize = dl*100*n_cell;
    // let second: isize = dc * n_cell;
    // total += droped + second;


fn can_push_up(grid: &[[u8;GRID_WIDTH*2];GRID_HEIGHT], line: usize, col: usize) -> bool {
    // we are ]
    match grid[line-1][col] {
        b'.' => match grid[line-1][col-1] {
            b'.' => true,
            b']' => can_push_up(grid, line-1, col-1),
            _ => false,
        }
        b']' => can_push_up(grid, line-1, col),
        b'[' => match grid[line-1][col-1] {
            b'.' => can_push_up(grid, line-1, col+1),
            b']' => can_push_up(grid, line-1, col-1) && can_push_up(grid, line-1, col+1),
            _ => false,
        }
        _ => false,
    }
}
fn do_push_up(grid: &mut [[u8;GRID_WIDTH*2];GRID_HEIGHT], line: usize, col: usize) {
    // we are ]
    match grid[line-1][col] {
        b'.' => match grid[line-1][col-1] {
            b']' => do_push_up(grid, line-1, col-1),
            _ => (),
        }
        b']' => do_push_up(grid, line-1, col),
        b'[' => match grid[line-1][col-1] {
            b'.' => do_push_up(grid, line-1, col+1),
            b']' => {do_push_up(grid, line-1, col-1) ; do_push_up(grid, line-1, col+1)},
            _ => (),
        }
        _ => (),
    };
    grid[line][col] = b'.';
    grid[line][col-1] = b'.';
    grid[line-1][col] = b']';
    grid[line-1][col-1] = b'[';
}
fn can_push_down(grid: &[[u8;GRID_WIDTH*2];GRID_HEIGHT], line: usize, col: usize) -> bool {
    // we are ]
    match grid[line+1][col] {
        b'.' => match grid[line+1][col-1] {
            b'.' => true,
            b']' => can_push_down(grid, line+1, col-1),
            _ => false,
        }
        b']' => can_push_down(grid, line+1, col),
        b'[' => match grid[line+1][col-1] {
            b'.' => can_push_down(grid, line+1, col+1),
            b']' => can_push_down(grid, line+1, col-1) && can_push_down(grid, line+1, col+1),
            _ => false,
        }
        _ => false,
    }
}
fn do_push_down(grid: &mut [[u8;GRID_WIDTH*2];GRID_HEIGHT], line: usize, col: usize) {
    match grid[line+1][col] {
        b'.' => match grid[line+1][col-1] {
            b']' => do_push_down(grid, line+1, col-1),
            _ => (),
        }
        b']' => do_push_down(grid, line+1, col),
        b'[' => match grid[line+1][col-1] {
            b'.' => do_push_down(grid, line+1, col+1),
            b']' => {do_push_down(grid, line+1, col-1) ; do_push_down(grid, line+1, col+1)},
            _ => (),
        }
        _ => (),
    };
    grid[line][col] = b'.';
    grid[line][col-1] = b'.';
    grid[line+1][col] = b']';
    grid[line+1][col-1] = b'[';
}

// fn show(grid: &mut [[u8;GRID_WIDTH*2];GRID_HEIGHT], line: usize, col: usize, byte: u8) {
//     // let old = grid[line][col];
//     // grid[line][col] = byte;
//     // for line in grid.iter() {
//     //     println!("{}", std::str::from_utf8(line).unwrap());
//     // }
//     // grid[line][col] = old;
//     // let ten_millis = time::Duration::from_millis(500);
//     // thread::sleep(ten_millis);
// }

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.as_bytes();
    let mut grid: [[u8;GRID_WIDTH*2];GRID_HEIGHT] = [[0u8;GRID_WIDTH*2];GRID_HEIGHT];
    let cmd = &input[GRID_END+1..];
    let mut pl = 0;
    let mut pc = 0;
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            match input[i*GRID_WIDTH+i + j] {
                b'@' => {
                    pl = i;
                    pc = j*2;
                    grid[i][j*2] = b'.';
                    grid[i][j*2+1] = b'.';
                },
                b'O' => {
                    grid[i][j*2] = b'[';
                    grid[i][j*2+1] = b']';
                },
                b'#' => {
                    grid[i][j*2] = b'#';
                    grid[i][j*2+1] = b'#';
                },
                b'.' => {
                    grid[i][j*2] = b'.';
                    grid[i][j*2+1] = b'.';
                },
                _ => unreachable!(),
            }
        }
    }
    let mut index = 0;
    for _ in 0..N_CMD_LINE {
        for _ in 0..N_CMD_PER_LINE {
            let cmdi = cmd[index];
            index += 1;
            // show(&mut grid, pl, pc, cmdi);
            match cmdi {
                b'^' => {
                    let new = (pl as isize - 1) as usize;
                    match grid[new][pc] {
                        b'[' => {
                            if can_push_up(&grid, new, pc+1) {
                                do_push_up(&mut grid, new, pc+1);
                                pl = new;
                            }
                        },
                        b']' => {
                            if can_push_up(&grid, new, pc) {
                                do_push_up(&mut grid, new, pc);
                                pl = new;
                            }
                        },
                        b'.' => {
                            pl = new;
                        },
                        _ => (),
                    }
                },
                b'v' => {
                    let new = (pl as isize + 1) as usize;
                    match grid[new][pc] {
                        b'[' => {
                            if can_push_down(&grid, new, pc+1) {
                                do_push_down(&mut grid, new, pc+1);
                                pl = new;
                            }
                        },
                        b']' => {
                            if can_push_down(&grid, new, pc) {
                                do_push_down(&mut grid, new, pc);
                                pl = new;
                            }
                        },
                        b'.' => {
                            pl = new;
                        },
                        _ => (),
                    }
                },
                b'>' => {
                    let new = (pc as isize + 1) as usize;
                    match grid[pl][new] {
                        b'[' => {
                            let mut free = new;
                            while grid[pl][free] == b'[' {
                                free += 2;
                            }

                            if grid[pl][free] == b'#' {
                                // show(&mut grid, pl, pc, b'x');
                                continue;
                            }

                            for i in ((new+1)..=free).step_by(2) {
                                grid[pl][i] = b'[';
                                grid[pl][i+1] = b']';
                            }
                            pc = new;
                            grid[pl][pc] = b'.';
                        },
                        b'.' => {
                            pc = new;
                        },
                        _ => (),
                    }
                },
                b'<' => {
                    let new = (pc as isize - 1) as usize;
                    match grid[pl][new] {
                        b']' => {
                            let mut free = new;
                            while grid[pl][free] == b']' {
                                free -= 2;
                            }

                            if grid[pl][free] == b'#' {
                                // show(&mut grid, pl, pc, b'x');
                                continue;
                            }

                            for i in (free..=new-1).step_by(2) {
                                grid[pl][i] = b'[';
                                grid[pl][i+1] = b']';
                            }
                            pc = new;
                            grid[pl][pc] = b'.';
                        },
                        b'.' => {
                            pc = new;
                        },
                        _ => (),
                    }
                },
                _ => unreachable!(),
            };
            // show(&mut grid, pl, pc, b'x');
        }
        index += 1;
    }
    let mut total = 0;
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH*2 {
            if grid[i][j] == b'[' {
                total += i*100 + j;
            }
        }
    }
    total
}
