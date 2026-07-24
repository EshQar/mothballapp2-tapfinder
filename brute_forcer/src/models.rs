use crate::iterators::{SignedCompositions, WeakCompositions, WeightIterable, WeightIterator};
use crate::math_datatypes::position_2d::Position2D;
use crate::math_datatypes::vec_position_2d::VecPosition2D;
use crate::math_datatypes::{TapOffset};

use std::ops::Mul;

pub trait Conglomerate 
where 
    for<'b> &'b Self::Offset: Mul<f32, Output = Self::Offset>,
{
    type Offset: TapOffset;
    fn weight_state(&self) -> Vec<&[isize]>;
    fn advance(&mut self) -> bool;
    fn update_max_counts(&mut self, max_counts: Vec<usize>);
    fn offset(&self) -> Self::Offset;
}

trait PoolIterable
where
    for<'b> &'b Self::Offset: Mul<f32, Output = Self::Offset>,
{
    type Offset: TapOffset;

    fn advance(&mut self) -> bool;
    fn reset(&mut self);
    fn state(&self) -> Self::Offset;
    fn weight_state(&self) -> &[isize];
}

struct PoolIterator<'a, O: TapOffset> where for<'b> &'b O: Mul<f32, Output = O> {
    pool: &'a [O],
    iterator: SignedCompositions,
}

impl<'a, O: TapOffset> PoolIterator<'a, O> where for<'b> &'b O: Mul<f32, Output = O> {
    fn new(pool: &'a [O], count: usize) -> Self {
        let k = pool.len();

        Self {
            pool,
            iterator: SignedCompositions::new(count as isize, k),
        }
    }
}

impl<O: TapOffset> PoolIterable for PoolIterator<'_, O>
where
    for<'b> &'b O: Mul<f32, Output = O>,
{
    type Offset = O;

    fn advance(&mut self)  -> bool {
        self.iterator.advance()
    }

    fn reset(&mut self) {
        self.iterator.reset();
    }

    fn state(&self) -> Self::Offset  {
        return self.pool.iter().zip(self.iterator.current().iter()).fold(Self::Offset::default(), |acc, (offset, weight)| {acc + offset * (*weight as f32)});
    }

    fn weight_state(&self) -> &[isize] {
        return self.iterator.current()
    }
}

pub struct PoolIteratorConglomerate<'a, O: TapOffset> where for<'b> &'b O: Mul<f32, Output = O> {
    iterators: Vec<WeightIterator>,
    pools: &'a [Vec<O>],
    count_of_pools: usize,
    pivot: usize,
}

impl<'a, O: TapOffset> PoolIteratorConglomerate<'a, O> where for<'b> &'b O: Mul<f32, Output = O> {
    pub fn new(pools: &'a Vec<Vec<O>>, is_reversible: &[bool]) -> Self {
        let count_of_pools = pools.len();
        let pool_sizes: Vec<usize> = pools.iter().map(|pool| pool.len()).collect();
        let iterators: Vec<WeightIterator> =
            is_reversible.into_iter().zip(pool_sizes.into_iter())
                .map(|(&rev, size)| {
                    if rev {
                        WeightIterator::Reversible(SignedCompositions::new(0, size))
                    } else {
                        WeightIterator::Irreversible(WeakCompositions::new(0, size))
                    }
                })
                .collect();

        PoolIteratorConglomerate { iterators, pools, count_of_pools, pivot: 0 }
    }
}

impl<'a, O: TapOffset> Conglomerate for PoolIteratorConglomerate<'a, O> where for<'b> &'b O: Mul<f32, Output = O> {
    type Offset = O;

    fn weight_state(&self) -> Vec<&[isize]> {
        self.iterators.iter().map(|iter| iter.current()).collect()
    }

    fn advance(&mut self) -> bool {
        if self.iterators[self.pivot].advance() {

            self.iterators[self.pivot].reset();
            self.pivot += 1;

            if self.pivot >= self.count_of_pools {
                return true
            }

            self.advance()
        } else {
            self.pivot = 0;
            return false
        }
    }

    fn update_max_counts(&mut self, max_counts: Vec<usize>) {
        self.pivot = 0;

        for (iterator, new_max_count) in self.iterators.iter_mut().zip(max_counts) {
            iterator.set_new_sum(new_max_count as isize);
        }
    }

    fn offset(&self) -> O {
        self.iterators.iter().zip(self.pools)
            .map(
                |(iter, pool)|
                    iter.current().iter().zip(pool).fold(
                        O::default(), |acc, (weight, offset)| acc + offset * (*weight as f32)
                    )
                )
            .fold(O::default(), |acc, offset| acc + offset)
    }
}



pub trait Goal<Offset: TapOffset> where for<'a> &'a Offset: Mul<f32, Output = Offset> {
    type Dist;
    type Pos;
    fn is_satisfied(&mut self, pos: &Offset) -> bool;
    fn get_dists(&self, pos: &Offset) -> Self::Dist;
    fn facings(&mut self) -> Vec<usize>;
}

pub struct SingleAxisGoal {
    start: f32,
    end: f32,
}

impl SingleAxisGoal { pub fn new(start: f32 , end: f32) -> Self { Self {start, end } } }


impl Goal<f32> for SingleAxisGoal {
    type Dist = (f32, f32);
    type Pos = f32;

    fn is_satisfied(&mut self, pos: &f32) -> bool {
        self.start < *pos && *pos < self.end
    }

    fn get_dists(&self, pos: &f32) -> Self::Dist {
        ((pos - self.start).abs(), (self.end - pos).abs())
    }

    fn facings(&mut self) -> Vec<usize> {
        Vec::new()
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

    fn is_satisfied(&mut self, pos: &Position2D) -> bool {
        (self.start.0 < pos.x && pos.x < self.end.0) && (self.start.1 < pos.z && pos.z < self.end.1)
    }

    fn get_dists(&self, pos: &Position2D) -> Self::Dist {
        ((pos.x - self.start.0).abs(), (pos.z - self.start.1).abs(), (self.end.0 - pos.x).abs(), (self.end.1 - pos.z).abs())
    }

    fn facings(&mut self) -> Vec<usize> {
        Vec::new()
    }
}

pub struct XRotationGoal {
    constraints: Vec<(f32, f32)>,
    facings: Vec<usize>,
}

impl XRotationGoal { 
    pub fn new(args: Vec<(f32, f32)>) -> Self { 
        let constraints = args.into_iter().map(|(a, b)| (a, b)).collect();
        Self { constraints, facings: Vec::new() } 
    }
}


impl Goal<VecPosition2D> for XRotationGoal where for<'a> &'a VecPosition2D: Mul<f32, Output = VecPosition2D> {
    type Dist = Vec<(f32, f32)>;
    type Pos = Position2D;

    fn is_satisfied(&mut self, pos: &VecPosition2D) -> bool {
        let mut has_succeeded: bool = false;
        for (i, (bound, position)) in self.constraints.iter().zip(pos.0.iter()).enumerate() {
            if bound.0 < position.x && position.x < bound.1 {
                self.facings.push(i);
                has_succeeded = true;
            }
        }

        has_succeeded
    }

    fn get_dists(&self, pos: &VecPosition2D) -> Self::Dist {
        let mut dists: Self::Dist = Vec::with_capacity(self.facings.len());
        for (bound, position) in self.constraints.iter().zip(pos.0.iter()) {
            dists.push(((position.x - bound.0).abs(), (bound.1 - position.x).abs()))
        }

        dists
    }

    fn facings(&mut self) -> Vec<usize> {
        std::mem::take(&mut self.facings)
    }
}

pub struct ZRotationGoal {
    constraints: Vec<(f32, f32)>,
    facings: Vec<usize>,
}

impl ZRotationGoal { 
    pub fn new(args: Vec<(f32, f32)>) -> Self { 
        let constraints = args.into_iter().map(|(a, b)| (a, b)).collect();
        Self { constraints, facings: Vec::new() } 
    }
}

impl Goal<VecPosition2D> for ZRotationGoal where for<'a> &'a VecPosition2D: Mul<f32, Output = VecPosition2D> {
    type Dist = Vec<(f32, f32)>;
    type Pos = Position2D;

    fn is_satisfied(&mut self, pos: &VecPosition2D) -> bool {
        let mut has_succeeded: bool = false;
        for (i, (bound, position)) in self.constraints.iter().zip(pos.0.iter()).enumerate() {
            if bound.0 < position.z && position.z < bound.1 {
                self.facings.push(i);
                has_succeeded = true;
            }
        }

        has_succeeded
    }

    fn get_dists(&self, pos: &VecPosition2D) -> Self::Dist {
        let mut dists: Self::Dist = Vec::with_capacity(self.facings.len());
        for (bound, position) in self.constraints.iter().zip(pos.0.iter()) {
            dists.push(((position.z - bound.0).abs(), (bound.1 - position.z).abs()))
        }

        dists
    }

    fn facings(&mut self) -> Vec<usize> {
        std::mem::take(&mut self.facings)
    }
}

pub struct MultiaxisRotationGoal {
    constraints: Vec<(f32, f32, f32, f32)>,
    facings: Vec<usize>,
}

impl MultiaxisRotationGoal { 
    pub fn new(args: Vec<((f32, f32), (f32, f32))>) -> Self { 
        let constraints = args.into_iter().map(|((a, b), (c, d))| (a, b, c, d)).collect();
        Self { constraints, facings: Vec::new() } 
    }
}

impl Goal<VecPosition2D> for MultiaxisRotationGoal where for<'a> &'a VecPosition2D: Mul<f32, Output = VecPosition2D> {
    type Dist = Vec<(f32, f32, f32, f32)>;
    type Pos = Position2D;

    fn is_satisfied(&mut self, pos: &VecPosition2D) -> bool {
        let mut has_succeeded: bool = false;
        for (i, (bound, position)) in self.constraints.iter().zip(pos.0.iter()).enumerate() {
            if (bound.0 < position.x && position.x < bound.2) && (bound.1 < position.z && position.z < bound.3) {
                self.facings.push(i);
                has_succeeded = true;
            }
        }

        has_succeeded
    }

    fn get_dists(&self, pos: &VecPosition2D) -> Self::Dist {
        let mut dists: Self::Dist = Vec::with_capacity(self.facings.len());
        for (bound, position) in self.constraints.iter().zip(pos.0.iter()) {
            dists.push(((position.z - bound.0).abs(), (position.x - bound.0).abs(), (bound.1 - position.x).abs(), (bound.1 - position.z).abs()))
        }

        dists
    }

    fn facings(&mut self) -> Vec<usize> {
        std::mem::take(&mut self.facings)
    }
}