use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity(pub u64);

impl From<u64> for Entity {
    fn from(id: u64) -> Self {
        Entity(id)
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity({})", self.0)
    }
}

impl Entity {
    pub fn none() -> Entity {
        Entity(0)
    }

    pub fn is_none(&self) -> bool {
        self.0 == Self::none().0
    }

    pub fn is_some(&self) -> bool {
        !self.is_none()
    }
}
