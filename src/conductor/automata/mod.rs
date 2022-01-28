use std::convert::From;
use std::ops::{Add, AddAssign};

use crate::common::Position;

pub mod random_walker;
pub mod sand;
pub mod water;

#[derive(Clone, Copy, Debug)]
pub enum Automata {
    RandomWalker,
    Water,
    Sand,
}

#[derive(Copy, Clone, Debug)]
pub struct Destination {
    pub x: i64,
    pub y: i64,
}

impl Destination {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub const fn distance(&self, rhs: &Self) -> i64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}

impl From<Position> for Destination {
    fn from(pos: Position) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl From<Destination> for Position {
    fn from(dest: Destination) -> Self {
        Self {
            x: dest.x,
            y: dest.y,
        }
    }
}

impl From<&Destination> for (i64, i64) {
    fn from(from: &Destination) -> (i64, i64) {
        (from.x, from.y)
    }
}

impl Add for Destination {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Destination {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
