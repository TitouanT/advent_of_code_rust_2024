pub mod corner;
use corner::{Corner, segment::OwnerId};
use super::heading::Heading;

#[derive(Debug)]
pub struct Cell<'a> {
    pub value : &'a str,
    pub ne: Corner,
    pub nw: Corner,
    pub se: Corner,
    pub sw: Corner,
}

impl<'a> Cell<'_> {
    pub fn new(value: &'a str) -> Cell<'a> {
        Cell {
            value,
            ne: Corner::empty(),
            nw: Corner::empty(),
            se: Corner::empty(),
            sw: Corner::empty(),
        }
    }

    fn n(&mut self, owner: OwnerId) {
        self.nw.activate(Heading::East, owner);
        self.ne.activate(Heading::West, owner);
    }
    fn s(&mut self, owner: OwnerId) {
        self.sw.activate(Heading::East, owner);
        self.se.activate(Heading::West, owner);
    }
    fn w(&mut self, owner: OwnerId) {
        self.nw.activate(Heading::South, owner);
        self.sw.activate(Heading::North, owner);
    }
    fn e(&mut self, owner: OwnerId) {
        self.ne.activate(Heading::South, owner);
        self.se.activate(Heading::North, owner);
    }

    pub fn activate(&mut self, heading: Heading, owner: OwnerId) {
        match heading {
            Heading::East => self.n(owner),
            Heading::West => self.s(owner),
            Heading::North => self.w(owner),
            Heading::South => self.e(owner),
        }
    }

    // alternative printing without the cell content.
    // pub const fn size() -> usize { 2 }

    // pub fn show(&self) -> [String; Cell::size()] {
    //     [
    //         format!("{}{}", self.nw.show(), self.ne.show()),
    //         format!("{}{}", self.sw.show(), self.se.show()),
    //     ]
    // }

    pub const fn size() -> usize { 3 }

    pub fn show(&self) -> [String; Cell::size()] {
        let n = match self.nw.east.owner_id { Some(owner) if self.nw.east == self.ne.west => corner::Side::new(Heading::East, owner).show(), _ => " ".to_string(), };
        let s = match self.sw.east.owner_id { Some(owner) if self.sw.east == self.se.west => corner::Side::new(Heading::East, owner).show(), _ => " ".to_string(), };
        let w = match self.nw.south.owner_id { Some(owner) if self.nw.south == self.sw.north => corner::Side::new(Heading::South, owner).show(), _ => " ".to_string(), };
        let e = match self.ne.south.owner_id { Some(owner) if self.ne.south == self.se.north => corner::Side::new(Heading::South, owner).show(), _ => " ".to_string(), };
        [
            format!("{}{}{}", self.nw.show(),     n     , self.ne.show()),
            format!("{}{}{}",              w, self.value, e             ),
            format!("{}{}{}", self.sw.show(),     s     , self.se.show()),
        ]
    }

    #[allow(dead_code)]
    fn is_valid(&self) -> bool {
        self.nw.east == self.ne.west
        &&
        self.sw.east == self.se.west
        &&
        self.nw.south == self.sw.north
        &&
        self.ne.south == self.se.north
    }
}
