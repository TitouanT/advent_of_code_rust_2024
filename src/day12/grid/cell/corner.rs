pub mod segment;
pub mod side;
use segment::{Segment, OwnerId};
use super::super::heading::Heading;
pub use side::Side;
#[derive(Debug)]
pub struct Corner {
    pub north: Segment,
    pub south: Segment,
    pub east: Segment,
    pub west: Segment
}

impl Corner {
    pub fn empty() -> Corner {
        Corner {
            north: Segment::new(None),
            south: Segment::new(None),
            east: Segment::new(None),
            west: Segment::new(None),
        }
    }
    fn assert_owner(&self, owner_id: OwnerId) {
        assert!(
            self.north.owner_id.is_none_or(|id| id == owner_id)
            &&
            self.south.owner_id.is_none_or(|id| id == owner_id)
            &&
            self.east.owner_id.is_none_or(|id| id == owner_id)
            &&
            self.west.owner_id.is_none_or(|id| id == owner_id)
        )
    }

    fn get_owner(&self) -> Option<OwnerId> {
        self.north.owner_id.or(self.south.owner_id).or(self.west.owner_id).or(self.east.owner_id)
    }

    pub fn show(&self) -> String {
        let color = if let Some(owner) = self.get_owner() {
            self.assert_owner(owner);
            ansi_term::Colour::Fixed((owner % u8::MAX as usize) as u8)
        }
        else {
            ansi_term::Colour::Fixed(0)
        };
        color.paint(self.as_str()).to_string()
    }

    pub fn as_str(&self) -> &str {
        match (self.north.as_bool(), self.south.as_bool(), self.east.as_bool(), self.west.as_bool()) {
            (false, false, false, false) => " ",
            (false, false, false,  true) => "╴",
            (false, false,  true, false) => "╶",
            (false, false,  true,  true) => "─",

            (false,  true, false, false) => "╷",
            (false,  true, false,  true) => "┐",
            (false,  true,  true, false) => "┌",
            (false,  true,  true,  true) => "┬",

            ( true, false, false, false) => "╵",
            ( true, false, false,  true) => "┘",
            ( true, false,  true, false) => "└",
            ( true, false,  true,  true) => "┴",

            ( true,  true, false, false) => "│",
            ( true,  true, false,  true) => "┤",
            ( true,  true,  true, false) => "├",
            ( true,  true,  true,  true) => "┼",
        }
    }

    pub fn activate(&mut self, direction: Heading, owner: OwnerId) {
        match direction {
            Heading::South => self.south.activate(owner),
            // _ => (),
            Heading::North => self.north.activate(owner),
            Heading::West  => self.west.activate(owner),
            Heading::East  => self.east.activate(owner),
        }
    }
}

