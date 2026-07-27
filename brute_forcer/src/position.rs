use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position2D {
    pub x: f32,
    pub z: f32,
}

impl Default for Position2D {
    fn default() -> Self {
        Position2D { x: 0.0f32, z: 0.0f32 }
    }
}


impl Add for Position2D {
    type Output = Position2D;

    fn add(self, rhs: Self) -> Self::Output {
        Position2D { x: self.x + rhs.x, z: self.z + rhs.z }
    }
}

impl Mul<f32> for Position2D {
    type Output = Position2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Position2D { x: self.x * rhs, z: self.z * rhs }
    }
}

impl Add for &Position2D {
    type Output = Position2D;

    fn add(self, rhs: Self) -> Self::Output {
        Position2D { x: self.x + rhs.x, z: self.z + rhs.z }
    }
}

impl Mul<f32> for &Position2D {
    type Output = Position2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Position2D { x: self.x * rhs, z: self.z * rhs }
    }
}

pub trait TapOffset:
    Copy +
    Clone + 
    Default + 
    Add<Output = Self> + Mul<f32, Output = Self> 
    {}

impl TapOffset for f32 {}

impl TapOffset for Position2D {}