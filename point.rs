// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num::Algebraic;

#[deriving(Eq)]
pub struct Point2D<T> {
    x: T,
    y: T
}

pub fn Point2D<T:Copy>(x: T, y: T) -> Point2D<T> {
    Point2D {x: x, y: y}
}


impl<T:Copy + Num> Add<Point2D<T>, Point2D<T>> for Point2D<T> {
    fn add(&self, other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x + other.x, self.y + other.y)
    }
}

impl<T:Copy + Num> Sub<Point2D<T>, Point2D<T>> for Point2D<T> {
    fn sub(&self, other: &Point2D<T>) -> Point2D<T> {
        Point2D(self.x - other.x, self.y - other.y)
    }
}

impl<T:Num> Point2D<T> {
    fn norm(&self) -> float {
        Algebraic::sqrt((self.x * self.x) + (self.y * self.y))
    }

    fn dist(&self, other: &Point2D<T>) -> float {
        (other - self).norm()
    }
}
