pub enum CatState {
    NotVisited,
    None,
    Cat(Cat),
}

impl CatState {
    pub fn has_visited(&self) -> bool {
        matches!(self, CatState::None | CatState::Cat(_))
    }
    pub fn get(&self) -> Option<&Cat> {
        match self {
            Self::Cat(cat) => Some(cat),
            _ => None,
        }
    }
}

#[derive(Default)]
pub struct Cat {}
