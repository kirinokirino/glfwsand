use std::cmp::Ordering;
use std::convert::From;
use std::ops::{Add, AddAssign};

use num_iter;

use crate::common::Position;

pub mod random_walker;
pub mod water;

pub enum Automata {
    RandomWalker,
    Water,
}

#[derive(Debug)]
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
