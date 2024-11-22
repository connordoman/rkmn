use crate::rkmn::Rkmn;

pub struct Party {
    rkmn: Vec<Rkmn>,
}

impl Party {
    pub fn len(&self) -> usize {
        self.rkmn.len()
    }

    pub fn get_last(&self) -> Option<&Rkmn> {
        self.rkmn.last()
    }

    pub fn new_test_party() -> Self {
        Self {
            rkmn: vec![Rkmn::new()],
        }
    }
}
