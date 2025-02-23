pub mod grid;
use grid::{Grid, GridPos, Loop};

fn both_part_cb(input: &str, edge_length: impl Fn(GridPos, GridPos) -> usize) -> usize {
    let mut grid = Grid::new(input);
    let mut loops_data = vec![];
    for line in 0..grid.lines.len() {
        let mut column = 0;
        let mut latest: Option<GridPos> = None;
        while let Some(new_col) = grid.walk_loop_cb(GridPos{ line, column }, |gp, owner| {
            match latest {
                None => {
                    if owner == loops_data.len() {
                        loops_data.push((0, 0));
                    }
                },
                Some(pgp) => {
                    loops_data[owner].0 += (pgp.column * gp.line) - (pgp.line * gp.column);
                    loops_data[owner].1 += edge_length(pgp, gp);
                }
            }
            latest = Some(gp);
        })  {
            column = new_col;
            latest = None;
        }
    }
    // for l in grid.show() {
    //     println!("{}",l);
    // }
    loops_data.into_iter().map(|(a, b)| a/2 * b).sum()
}

fn both_part_iter(input: &str, edge_length: impl Fn(GridPos, GridPos) -> usize) -> usize {
    let mut grid = Grid::new(input);
    let mut loops_data = vec![];

    // ************************
    // ** vec                **
    // **                    **
    // ** here edges is a vec, so the current loop has already been visited, and stored.
    for Loop { owner, edges } in grid.iter() {
        let owner = match owner {
            None => { loops_data.push((0, 0)); loops_data.len() - 1 },
            Some(owner) => owner
        };
        for (a, b) in edges {
            loops_data[owner].0 += (a.column * b.line) - (a.line * b.column);
            loops_data[owner].1 += edge_length(a, b);
        }
    }
    // **                    **
    // ************************
    // ************************
    // ** extern attach      **
    // **                    **
    // ** doesnt work because grid.iter() already borrows grid mutably.
    // for edges in grid.iter() {
    //     let owner = match edges.owner {
    //         None => { loops_data.push((0, 0)); loops_data.len() - 1 },
    //         Some(owner) => owner
    //     };
    //     for (a, b) in edges.attach(&mut grid) {
    //         loops_data[owner].0 += (a.column * b.line) - (a.line * b.column);
    //         loops_data[owner].1 += edge_length(a, b);
    //     }
    // }
    // **                    **
    // ************************
    // ************************
    // ** intern attach      **
    // **                    **
    // ** doesnt work because edges here has an internal mutable reference to grid and that clashes
    // ** with the borrow that grid.iter() has on grid.
    // for edges in grid.iter() {
    //     let owner = match edges.owner {
    //         None => { loops_data.push((0, 0)); loops_data.len() - 1 },
    //         Some(owner) => owner
    //     };
    //     for (a, b) in edges {
    //         loops_data[owner].0 += (a.column * b.line) - (a.line * b.column);
    //         loops_data[owner].1 += edge_length(a, b);
    //     }
    // }
    // **                    **

    // for l in grid.show() {
    //     println!("{l}");
    // }
    loops_data.into_iter().map(|(a, b)| a/2 * b).sum()
}



#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    both_part_cb(input, |a, b| a.line.abs_diff(b.line) + a.column.abs_diff(b.column))
}


#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    both_part_cb(input, |_, _| 1)
}


pub fn part1_iter(input: &str) -> usize {
    both_part_iter(input, |a, b| a.line.abs_diff(b.line) + a.column.abs_diff(b.column))
}


pub fn part2_iter(input: &str) -> usize {
    both_part_iter(input, |_, _| 1)
}

