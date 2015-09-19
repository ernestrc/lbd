use super::dal::Dal;

pub struct System {
    dal: Dal
}

impl System {
    pub fn new(dal: Dal) -> System {
        System {
            dal: dal
        }
    }
}
