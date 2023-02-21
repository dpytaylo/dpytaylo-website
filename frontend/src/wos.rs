use crate::world::World;

#[derive(Default)]
pub struct Wos {
    pub worlds: Vec<World>,
}

impl Wos {
    pub fn new() -> Self {
        Self {
            worlds: Vec::new(),
        }
    }
}