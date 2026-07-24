use crate::math_datatypes::position_2d::Position2D;

use std::ops::{Add, Mul};
use itertools::{EitherOrBoth::{*}, Itertools,};

#[derive(Clone, Debug, PartialEq)]
pub struct VecPosition2D(pub Vec<Position2D>);

impl Default for VecPosition2D {
    fn default() -> Self {
        Self(Vec::new())
    }
}


impl Add for VecPosition2D {
    type Output = VecPosition2D;

    fn add(self, rhs: Self) -> Self::Output {
        VecPosition2D(self.0.iter().zip_longest(rhs.0.iter()).map(|pair| 
            match pair { 
                Both(pos1, pos2) => { *pos1 + *pos2 }
                Left(pos) => { *pos + Position2D::default() }
                Right(pos) => { *pos + Position2D::default() }
            } ).collect())
    }
}

impl Mul<f32> for VecPosition2D {
    type Output = VecPosition2D;

    fn mul(self, rhs: f32) -> Self::Output {
        VecPosition2D(self.0.into_iter().map(|pos| pos * rhs).collect())
    }
}

impl Add for &VecPosition2D {
    type Output = VecPosition2D;

    fn add(self, rhs: Self) -> Self::Output {
        VecPosition2D(self.0.iter().zip_longest(rhs.0.iter()).map(|pair| 
            match pair { 
                Both(pos1, pos2) => { *pos1 + *pos2 }
                Left(pos) => { *pos + Position2D::default() }
                Right(pos) => { *pos + Position2D::default() }
            } ).collect())
    }
}

impl Mul<f32> for &VecPosition2D {
    type Output = VecPosition2D;

    fn mul(self, rhs: f32) -> Self::Output {
        VecPosition2D(self.0.iter().map(|pos| *pos * rhs).collect())
    }
}