pub mod position_2d;
pub mod vec_position_2d;

use crate::math_datatypes::position_2d::Position2D;
use crate::math_datatypes::vec_position_2d::VecPosition2D;

use std::ops::{Add, Mul};

pub trait TapOffset: 
    Clone + 
    Default + 
    Add<Output = Self> + Mul<f32, Output = Self> 
    where 
        for<'a> &'a Self: Mul<f32, Output = Self> 
    {}

impl TapOffset for f32 {}

impl TapOffset for Position2D {}

impl TapOffset for VecPosition2D {}