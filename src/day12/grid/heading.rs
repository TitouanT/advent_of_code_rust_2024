use super::pos::delta_pos::DeltaPos;
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Heading {
    North,
    South,
    East,
    West,
}

impl Heading {
    pub fn left(&self) -> DeltaPos {
        match self {
            Heading::East  => DeltaPos{ heading: Heading::North, dl: -1, dc:  1 },
            Heading::South => DeltaPos{ heading: Heading::East , dl:  1, dc:  1 },
            Heading::West  => DeltaPos{ heading: Heading::South, dl:  1, dc: -1 },
            Heading::North => DeltaPos{ heading: Heading::West , dl: -1, dc: -1 },
        }
    }
    pub fn right(&self) -> DeltaPos {
        match self {
            Heading::East  => DeltaPos{ heading: Heading::South, dl:  0, dc:  0 },
            Heading::South => DeltaPos{ heading: Heading::West , dl:  0, dc:  0 },
            Heading::West  => DeltaPos{ heading: Heading::North, dl:  0, dc:  0 },
            Heading::North => DeltaPos{ heading: Heading::East , dl:  0, dc:  0 },
        }
    }
    pub fn forward(&self) -> DeltaPos {
        match self {
            Heading::East  => DeltaPos{ heading: Heading::East , dl:  0, dc:  1 },
            Heading::South => DeltaPos{ heading: Heading::South, dl:  1, dc:  0 },
            Heading::West  => DeltaPos{ heading: Heading::West , dl:  0, dc: -1 },
            Heading::North => DeltaPos{ heading: Heading::North, dl: -1, dc:  0 },
        }
    }
    pub fn corner(&self) -> DeltaPos {
        match self {
            Heading::East  => DeltaPos{ heading: Heading::East , dl:  0, dc:  0 },
            Heading::South => DeltaPos{ heading: Heading::South, dl:  0, dc:  1 },
            Heading::West  => DeltaPos{ heading: Heading::West , dl:  1, dc:  1 },
            Heading::North => DeltaPos{ heading: Heading::North, dl:  1, dc:  0 },
        }
    }
}
