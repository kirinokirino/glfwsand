use crate::automata::Destination;
use crate::common::Position;
use fastrand;

const SPEED: i64 = 1;
pub fn update(pos: &Position) -> Destination {
    Destination::from(
        *pos + Position::new(fastrand::i64(-SPEED..=SPEED), fastrand::i64(-SPEED..=SPEED)),
    )
}
