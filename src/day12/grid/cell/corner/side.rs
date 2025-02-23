use super::{Corner, Heading, Segment, OwnerId};
pub struct Side {
    sub_corner: Corner,
}

impl Side {
    pub fn show(&self) -> String {
        self.sub_corner.show()
    }

    pub fn new(heading: Heading, owner: OwnerId) -> Side {
        let segment = Segment::new(Some(owner));
        match heading {
            Heading::South | Heading::North => Side {
                sub_corner: Corner {
                    north: segment,
                    south: segment,
                    ..Corner::empty()
                },
            },
            _ => Side {
                sub_corner: Corner {
                    east: segment,
                    west: segment,
                    ..Corner::empty()
                },
            }
        }
    }
}
