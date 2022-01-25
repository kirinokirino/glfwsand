use crate::common::Position;
use fastrand;

pub struct Walker {
    pub position: Position,
}

impl Walker {
    pub const fn new(position: Position) -> Self {
        Self { position }
    }

    pub fn update(&mut self) {
        self.position += Position::new(fastrand::i64(-1..2), fastrand::i64(-1..2));
    }

    pub fn draw(&mut self /* viewport */) {
        todo!();
    }
}
