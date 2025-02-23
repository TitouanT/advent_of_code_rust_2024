#[allow(unused_imports)]
use std::{thread, time};
pub mod cell;
pub mod pos;
pub mod heading;
pub mod iterator;
use cell::Cell;
use heading::Heading;
use pos::Pos;
pub use pos::GridPos;
pub use iterator::Loop;

use cell::corner::segment::OwnerId;


pub struct Grid<'a> {
    pub lines : Vec<Vec<Cell<'a>>>,
    n_loops: OwnerId,
}

impl<'a> Grid<'a> {
    pub fn new(grid: &'a str) -> Grid<'a> {
        Grid {
            lines: grid.split("\n").filter(|l| !l.is_empty()).map(|line| line.split_inclusive(|_|true).map(Cell::new).collect()).collect(),
            n_loops: 0,
        }
    }

    fn get(&self, pos: &Pos) -> Option<&Cell<'a>> {
        self.lines.get(pos.line)?.get(pos.column)
    }
    fn get_mut(&mut self, pos: &Pos) -> Option<&mut Cell<'a>> {
        self.lines.get_mut(pos.line)?.get_mut(pos.column)
    }

    fn connect(&mut self, source: &Pos, destination: &Pos, owner: OwnerId) {
        match destination.heading {
            Heading::South => {
                if let Some(c) = self.get_mut(source)      { c.se.activate(Heading::South, owner) }
                if let Some(c) = self.get_mut(destination) { c.ne.activate(Heading::North, owner) }
            },
            Heading::North => {
                if let Some(c) = self.get_mut(source)      { c.nw.activate(Heading::North, owner) }
                if let Some(c) = self.get_mut(destination) { c.sw.activate(Heading::South, owner) }
            },
            // _ => (),
            Heading::West => {
                if let Some(c) = self.get_mut(source)      { c.sw.activate(Heading::West, owner) }
                if let Some(c) = self.get_mut(destination) { c.se.activate(Heading::East, owner) }
            },
            Heading::East => {
                if let Some(c) = self.get_mut(source)      { c.ne.activate(Heading::East, owner) }
                if let Some(c) = self.get_mut(destination) { c.nw.activate(Heading::West, owner) }
            },
        }
    }

    fn activate(&mut self, pos: &Pos, owner: OwnerId) {
        let mut show = false;
        if let Some(c) = self.get_mut(pos) {
            show = true;
            c.activate(pos.heading, owner)
        }
        if show { self.show_wait() }
    }

    pub fn show(&self) -> Vec<String> {
        let mut repr: Vec<String> = vec!();
        for line in self.lines.iter() {
            let cells_strs = line.iter().map(|c| c.show()).collect::<Vec<_>>();
            for i in 0..Cell::size() {
                let str = cells_strs.iter().map(|cr| cr[i].clone()).collect::<Vec<String>>().join("");
                repr.push(str);
            }
        }
        repr
    }

    fn show_wait(&self) {
        // for line in self.show() { println!("{}", line) }
        // let ten_millis = time::Duration::from_millis(500);
        // thread::sleep(ten_millis);
    }
}

// call back based walk
impl Grid<'_> {
    // walk around a whole loop, calling cb on each corner.
    pub fn walk_loop_cb(&mut self, start: GridPos, mut cb: impl FnMut (GridPos, OwnerId) ) -> Option<usize> {
        let value = self.lines[start.line][start.column].value;
        let (start, mut current, ret_val, owner) = {
            let start = Pos { line: start.line, column: start.column, heading: Heading::East };
            match self.get(&start) {
                None => { panic!() }
                // The main contour of that region is already found
                // we need look for the next region.
                Some(start_cell) if start_cell.nw.south.as_bool() => {
                    let mut current = start;
                    let mut candidate = current.forward();
                    let mut ret_val = None;
                    let mut is_positive = start_cell.ne.south.as_bool();
                    while let Some(next_cell) = self.get(&candidate) {
                        if next_cell.value != value {
                            ret_val = Some(candidate.column);
                            break
                        }
                        is_positive = next_cell.ne.south.as_bool();
                        current = candidate;
                        candidate = current.forward();
                    }
                    // we found the edge of the region we are in, so nothing needs to be done
                    if ret_val.is_none() || is_positive { return ret_val }
                    // we found a hole in the main the region, so we need to walk around that hole
                    // before going to the next.
                    else {
                        let owner = start_cell.nw.south.owner_id.unwrap();
                        let start = Pos {heading: Heading::South, ..current };
                        cb(GridPos::from(start), owner);
                        ( start, start, ret_val, owner)
                    }
                }
                // Main contour of a region needs to be walked around, we start the walk here while
                // looking for the region that will need to be treated after.
                _ => {
                    let owner = self.n_loops;
                    self.n_loops += 1;
                    self.activate(&start, owner);
                    cb(GridPos::from(start), owner);
                    let mut current = start;
                    let mut candidate = start.forward();
                    let mut ret_val = None;
                    while let Some(next_cell) = self.get(&candidate) {
                        if next_cell.value != value {
                            ret_val = Some(candidate.column);
                            break
                        }
                        self.connect(&current, &candidate, owner);
                        self.activate(&candidate, owner);
                        current = candidate;
                        candidate = current.forward();
                    }
                    current = current.right();
                    cb(GridPos::from(current), owner);
                    ( start, current, ret_val, owner )
                }
            }
        };

        loop {
            self.activate(&current, owner);
            let forward = current.forward();
            let right = current.right();
            let left = current.left();
            current = if self.get(&forward).is_some_and(|c| c.value == value) {
                self.connect(&current, &forward, owner);
                if self.get(&left).is_some_and(|c| c.value == value) {
                    self.connect(&forward, &left, owner);
                    cb(GridPos::from(left), owner);
                    left
                }
                else { forward }
            }
            else { cb(GridPos::from(right), owner); right };
            if current == start {
                break
            }

        }
        ret_val
    }
}

