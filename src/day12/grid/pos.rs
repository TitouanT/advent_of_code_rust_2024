use std::ops::Add;
pub mod delta_pos;
pub mod grid_pos;
pub use delta_pos::DeltaPos;
pub use grid_pos::GridPos;

use super::heading::Heading;
impl Pos {
    pub fn left   (self) -> Pos { self + self.heading.left()    }
    pub fn right  (self) -> Pos { self + self.heading.right()   }
    pub fn forward(self) -> Pos { self + self.heading.forward() }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
    pub heading: Heading,
}

impl Add<DeltaPos> for Pos {
    type Output = Pos;
    fn add(self, delta: DeltaPos) -> Self::Output {
        Pos {
            line: (self.line as isize + delta.dl) as usize,
            column: (self.column as isize + delta.dc) as usize,
            heading: delta.heading,
        }
    }
}

