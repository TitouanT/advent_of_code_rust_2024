use super::{Grid, OwnerId, Heading, Pos, GridPos};
// iterator based walk
impl<'a> Grid<'a> {

    pub fn iter<'handle>(&'handle mut self) -> LoopsIterator<'a, 'handle> {
        LoopsIterator {
            grid: self,
            search_head: Some(GridPos {line: 0, column: 0}),
            nloops: 0,
        }
    }
    // first edge is special cased because we walk along it but also provide the next spot from
    // which to start looking for the next loop
    #[allow(dead_code)]
    fn walk_loop_first_edge_detached(&self, start: GridPos) -> (Option<GridPos>, Option<DetachedLoopIterator>) {
        let gstart = start;
        let start = Pos {line: start.line, column: start.column, heading: Heading::East};
        let value = self.lines[start.line][start.column].value;
        match self.get(&start) {
            None => { panic!() }
            // we need to look for the next region, or a hole.
            Some(start_cell) => {
                let mut current = start;
                let mut candidate = start.forward();
                let mut ret_val = None;
                while let Some(next_cell) = self.get(&candidate) {
                    if next_cell.value != value {
                        ret_val = Some(GridPos {column: candidate.column, ..gstart});
                        break
                    }
                    current = candidate;
                    candidate = candidate.forward();
                }
                (
                    ret_val.or_else(|| {
                        // we found the edge of the map, so we need to continue on the next line
                        if start.line + 1 < self.lines.len() {
                            Some(GridPos {
                                line: start.line+1,
                                column: 0,
                            })
                        }
                        // no more lines, the search stops
                        else {
                            None
                        }
                    }),
                    match self.get(&current) {
                        // The main contour of that region is already found
                        Some(cell) if cell.ne.south.as_bool() => None,
                        _ => {
                            let owner = start_cell.nw.south.owner_id;
                            Some(DetachedLoopIterator {
                                // owner ? Main contour of a region needs to be walked around : Contour of a hole in a region needs to be walked around
                                start: if owner.is_none() { start } else { Pos { heading: Heading::South, ..current } },
                                owner,
                                latest: None,
                            })
                        }
                    }
                )
            }
        }
    }

    // first edge is special cased because we walk along it but also provide the next spot from
    // which to start looking for the next loop
    fn walk_loop_first_edge_attached<'mutable_grid_handle>(&'mutable_grid_handle mut self, start: GridPos) -> (Option<GridPos>, Option<LoopIterator<'a, 'mutable_grid_handle>>) {
        let gstart = start;
        let start = Pos {line: start.line, column: start.column, heading: Heading::East};
        let value = self.lines[start.line][start.column].value;
        // let start = Pos { line: start_line, column: start_column, heading: Heading::East };
        match self.get(&start) {
            None => { panic!() }
            // we need to look for the next region, or a hole.
            Some(start_cell) => {
                let mut current = start;
                let mut candidate = start.forward();
                let mut ret_val = None;
                while let Some(next_cell) = self.get(&candidate) {
                    if next_cell.value != value {
                        ret_val = Some(GridPos {column: candidate.column, ..gstart});
                        break
                    }
                    current = candidate;
                    candidate = candidate.forward();
                }
                (
                    ret_val.or_else(|| {
                        // we found the edge of the map, so we need to continue on the next line
                        if start.line + 1 < self.lines.len() {
                            Some(GridPos {
                                line: start.line+1,
                                column: 0,
                            })
                        }
                        // no more lines, the search stops
                        else {
                            None
                        }
                    }),
                    match self.get(&current) {
                        // The main contour of that region is already found
                        Some(cell) if cell.ne.south.as_bool() => None,
                        _ => {
                            let owner = start_cell.nw.south.owner_id;
                            Some(LoopIterator {
                                grid: self,
                                // owner ? Main contour of a region needs to be walked around : Contour of a hole in a region needs to be walked around
                                start: if owner.is_none() { start } else { Pos { heading: Heading::South, ..current } },
                                owner,
                                latest: None,
                            })
                        }
                    }
                )
            }
        }
    }

    // walk along a single edge of a loop
    fn walk_loop_single_edge(&mut self, start: Pos, owner: OwnerId) -> Pos {
        let value = self.get(&start).unwrap().value;
        let mut current = start;
        loop {
            self.activate(&current, owner);
            let forward = current.forward();
            let right = current.right();
            let left = current.left();
            if self.get(&forward).is_some_and(|c| c.value == value) {
                self.connect(&current, &forward, owner);
                if self.get(&left).is_some_and(|c| c.value == value) {
                    self.connect(&forward, &left, owner);
                    return left;
                }
                else { current = forward }
            }
            else {
                return right
            }
        }
    }
}


#[derive(Debug)]
pub struct Loop {
    pub owner: Option<OwnerId>,
    pub edges: Vec<(GridPos, GridPos)>,
}

pub struct LoopsIterator<'grid_content, 'mutable_grid_handle> {
    grid: &'mutable_grid_handle mut Grid<'grid_content>,
    search_head: Option<GridPos>,
    nloops: OwnerId,
}

// impl<????> Iterator for LoopsIterator<????> {
//     type Item = LoopIterator<'????, '????>;
//     fn next(&mut self) -> Option<LoopIterator<'????, '???>> {
//         let mut ret_loop = None;
//         while ret_loop.is_none() {
//             let head = self.search_head?;
//             let (next_head, next_loop) = self.grid.walk_loop_first_edge_detached(head);
//             self.search_head = next_head;
//             ret_loop = next_loop;
//         }
//         Some(ret_loop?.attach(&'??? mut self.grid))
//     }
// }

impl Iterator for LoopsIterator<'_, '_> {
    type Item = Loop;
    fn next(&mut self) -> Option<Self::Item>{
        let mut ret_loop = None;
        while ret_loop.is_none() {
            let head = self.search_head?;
            let (next_head, next_loop) = self.grid.walk_loop_first_edge_attached(head);
            self.search_head = next_head;
            ret_loop = next_loop;
        }
        let mut ret_loop = ret_loop?;
        let owner = ret_loop.owner;
        if owner.is_none() {
            ret_loop.owner = Some(self.nloops);
            self.nloops += 1;
        }
        Some(Loop {owner, edges: ret_loop.collect() })
    }
}

// impl Iterator for LoopsIterator<'_, '_> {
//     type Item = DetachedLoopIterator;
//     fn next(&mut self) -> Option<Self::Item>{
//         let mut ret_loop = None;
//         while ret_loop.is_none() {
//             let head = self.search_head?;
//             let (next_head, next_loop) = self.grid.walk_loop_first_edge_detached(head);
//             self.search_head = next_head;
//             ret_loop = next_loop;
//         }
//         ret_loop
//     }
// }

pub struct DetachedLoopIterator {
    start: Pos,
    latest: Option<(Pos, GridPos)>,
    pub owner: Option<OwnerId>,
}

impl DetachedLoopIterator {
    pub fn attach<'content, 'handle>(self, grid: &'handle mut Grid<'content>) -> LoopIterator<'content, 'handle> {
        LoopIterator {
            grid,
            start: self.start,
            latest: self.latest,
            owner: self.owner,
        }
    }
}

pub struct LoopIterator<'content, 'handle> {
    grid: &'handle mut Grid<'content>,
    start: Pos,
    latest: Option<(Pos, GridPos)>,
    pub owner: Option<OwnerId>,
}

impl Iterator for LoopIterator<'_, '_> {
    type Item = (GridPos, GridPos);
    fn next(&mut self) -> Option<Self::Item> {
        let (prev, gprev) = match self.latest {
            Some((start, _)) if start == self.start => return None,
            Some(prev) => prev,
            None => (self.start, GridPos::from(self.start)),
        };
        let curr = self.grid.walk_loop_single_edge(prev, self.owner?);
        let gcurr = GridPos::from(curr);
        self.latest = Some((curr, gcurr));
        Some((gprev, gcurr))
    }
}
