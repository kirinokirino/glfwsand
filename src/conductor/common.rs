use std::convert::From;
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub const fn distance(&self, rhs: &Self) -> i64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }

    // Returns a vector of positions starting from the end and going back to the start in a straight line.
    pub fn straight_line(self, end_point: Self) -> Vec<Self> {
        let (dest_x, dest_y) = end_point.into();
        let (start_x, start_y) = self.into();
        let (delta_x, delta_y) = ((dest_x - start_x), (dest_y - start_y));
        if delta_x == 0 && delta_y == 0 {
            return Vec::new();
        }

        let spacing = if delta_x == 0 || delta_y == 0 {
            1.0
        } else {
            let dx = delta_x.abs() as f64;
            let dy = delta_y.abs() as f64;
            (dx / dy).max(dy / dx)
        };

        let len = delta_x.abs().max(delta_y.abs());
        let mut result: Vec<Self> = Vec::with_capacity(len as usize);

        let x_step = delta_x.signum();
        let x_iter = num_iter::range_step_inclusive(start_x, dest_x, x_step);
        let y_step = delta_y.signum();
        let y_iter = num_iter::range_step_inclusive(start_y, dest_y, y_step);

        let mut counter = 0.0;
        if delta_x.abs() > delta_y.abs() {
            let mut y = start_y;
            let last = vec![dest_y];
            let mut short_range = y_iter.chain(last.iter().cycle().copied());
            result.extend(x_iter.map(|x| {
                counter += 1.0;
                while counter > spacing {
                    y = short_range.next().expect("iterator should be endless");
                    counter -= spacing;
                }
                Self::new(x, y)
            }));
        } else {
            let mut x = start_x;
            let last = vec![dest_x];
            let mut short_range = x_iter.chain(last.iter().cycle().copied());
            result.extend(y_iter.map(|y| {
                counter += 1.0;
                while counter > spacing {
                    x = short_range.next().expect("iterator should be endless");
                    counter -= spacing;
                }
                Self::new(x, y)
            }));
        };
        result
    }
}

impl From<Position> for (i64, i64) {
    fn from(from: Position) -> (i64, i64) {
        (from.x, from.y)
    }
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
