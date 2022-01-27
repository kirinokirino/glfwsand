use crate::automata::Destination;
use crate::common::Position;
use fastrand;

pub fn update(pos: &Position) -> Destination {
    Destination::from(*pos + Position::new(fastrand::i64(-1..2), 1))
}
