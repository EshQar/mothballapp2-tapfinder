
use std::ops::Mul;

use crate::position::{TapOffset, Position2D};

pub trait Goal<Offset: TapOffset> {
    type Dist;
    type Pos;
}

pub trait NoRotationGoal<Offset: TapOffset>: Goal<Offset> {
    fn is_satisfied(&mut self, pos: &Offset) -> bool;
    fn get_dists(&self, pos: &Offset) -> Self::Dist;
}

pub trait RotationGoal<Offset: TapOffset>: Goal<Offset> {
    fn is_satisfied(&mut self, pos: &[Offset]) -> bool;
    fn fetch_best_tap_data(&self, pos: &[Offset]) -> (Self::Dist, Self::Pos, Vec<usize>);
}

pub struct SingleAxisGoal {
    start: f32,
    end: f32,
}

impl SingleAxisGoal { pub fn new(start: f32 , end: f32) -> Self { Self {start, end } } }

impl Goal<f32> for SingleAxisGoal {
    type Dist = (f32, f32);
    type Pos = f32;
}

impl NoRotationGoal<f32> for SingleAxisGoal {
    fn is_satisfied(&mut self, pos: &f32) -> bool {
        self.start < *pos && *pos < self.end
    }

    fn get_dists(&self, pos: &f32) -> Self::Dist {
        ((pos - self.start).abs(), (self.end - pos).abs())
    }
}


pub struct MultiAxisGoal {
    start: (f32, f32),
    end: (f32, f32),
}

impl MultiAxisGoal { pub fn new(start: (f32, f32), end: (f32, f32)) -> Self { Self {start, end } } }


impl Goal<Position2D> for MultiAxisGoal where for<'a> &'a Position2D: Mul<f32, Output = Position2D> {
    type Dist = (f32, f32, f32, f32);
    type Pos = Position2D;
}

impl NoRotationGoal<Position2D> for MultiAxisGoal {
    fn is_satisfied(&mut self, pos: &Position2D) -> bool {
        (self.start.0 < pos.x && pos.x < self.end.0) && (self.start.1 < pos.z && pos.z < self.end.1)
    }

    fn get_dists(&self, pos: &Position2D) -> Self::Dist {
        ((pos.x - self.start.0).abs(), (pos.z - self.start.1).abs(), (self.end.0 - pos.x).abs(), (self.end.1 - pos.z).abs())
    }
}

pub struct XRotationGoal {
    constraints: Vec<(f32, f32)>,
}

impl XRotationGoal { 
    pub fn new(args: Vec<(f32, f32)>) -> Self { 
        Self { constraints: args } 
    }
}


impl Goal<Position2D> for XRotationGoal {
    type Dist = (f32, f32);
    type Pos = f32;
}

impl RotationGoal<Position2D> for XRotationGoal {
    fn is_satisfied(&mut self, pos: &[Position2D]) -> bool {
        self.constraints.iter().zip(pos.iter()).any(|(bound, position)| bound.0 < position.x && position.x < bound.1)
    }

    fn fetch_best_tap_data(&self, pos: &[Position2D]) -> (Self::Dist, Self::Pos, Vec<usize>) {
        let mut facings = Vec::new();

        for (i, (bound, position)) in self.constraints.iter().zip(pos.iter()).enumerate() {
            if bound.0 < position.x && position.x < bound.1 {
                facings.push(i);
            }
        }

        let index = facings[facings.len() / 2];
        let position = pos[index].x;
        let bound = self.constraints[index];
        let dists = ((position - bound.0).abs(), (bound.1 - position).abs());

        (dists, position, facings)
    }
}

pub struct ZRotationGoal {
    constraints: Vec<(f32, f32)>,
}

impl ZRotationGoal { 
    pub fn new(args: Vec<(f32, f32)>) -> Self { 
        Self { constraints: args } 
    }
}

impl Goal<Position2D> for ZRotationGoal {
    type Dist = (f32, f32);
    type Pos = f32;
}

impl RotationGoal<Position2D> for ZRotationGoal {
    fn is_satisfied(&mut self, pos: &[Position2D]) -> bool {
        self.constraints.iter().zip(pos.iter()).any(|(bound, position)| bound.0 < position.z && position.z < bound.1)
    }

    fn fetch_best_tap_data(&self, pos: &[Position2D]) -> (Self::Dist, Self::Pos, Vec<usize>) {
        let mut facings = Vec::new();

        for (i, (bound, position)) in self.constraints.iter().zip(pos.iter()).enumerate() {
            if bound.0 < position.z && position.z < bound.1 {
                facings.push(i);
            }
        }

        let index = facings[facings.len() / 2];
        let position = pos[index].z;
        let bound = self.constraints[index];
        let offset = ((position - bound.0).abs(), (bound.1 - position).abs());

        (offset, position, facings)
    }
}

pub struct MultiAxisRotationGoal {
    constraints: Vec<(f32, f32, f32, f32)>,
}

impl MultiAxisRotationGoal { 
    pub fn new(args: Vec<((f32, f32), (f32, f32))>) -> Self { 
        let constraints = args.into_iter().map(|((a, b), (c, d))| (a, b, c, d)).collect();
        Self { constraints } 
    }
}

impl Goal<Position2D> for MultiAxisRotationGoal {
    type Dist = (f32, f32, f32, f32);
    type Pos = Position2D;
}

impl RotationGoal<Position2D> for MultiAxisRotationGoal {
    fn is_satisfied(&mut self, pos: &[Position2D]) -> bool {
        self.constraints.iter().zip(pos.iter()).any(|(bound, position)| (bound.0 < position.x && position.x < bound.2) && (bound.1 < position.z && position.z < bound.3))
    }

    fn fetch_best_tap_data(&self, pos: &[Position2D]) -> (Self::Dist, Self::Pos, Vec<usize>) {
        let mut facings = Vec::new();

        for (i, (bound, position)) in self.constraints.iter().zip(pos.iter()).enumerate() {
            if (bound.0 < position.x && position.x < bound.2) && (bound.1 < position.z && position.z < bound.3) {
                facings.push(i);
            }
        }

        let index = facings[facings.len() / 2];
        let position = pos[index];
        let bound = self.constraints[index];
        let dists = ((position.x - bound.0).abs(), (position.z - bound.1).abs(), (bound.2 - position.x).abs(), (bound.3 - position.z).abs());

        (dists, position, facings)
    }
}