pub type OwnerId = usize;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Segment {
    pub owner_id: Option<OwnerId>,
}

impl Segment {
    pub fn activate(&mut self, owner_id: OwnerId) {
        assert!(self.owner_id.is_none());
        self.owner_id = Some(owner_id);
    }

    pub fn as_bool(&self) -> bool {
        self.owner_id.is_some()
    }

    pub fn new(owner: Option<OwnerId>) -> Segment {
        Segment {
            owner_id: owner,
        }
    }
}
