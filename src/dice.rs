use rand::{thread_rng, Rng};

pub struct Dice {
    faces: u32,
}

impl Dice {
    pub fn new(faces: u32) -> Dice {
        Dice { faces }
    }

    pub fn roll(&self) -> u32 {
        thread_rng().gen_range(1, self.faces)
    }
}
