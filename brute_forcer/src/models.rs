use crate::iterators::{WeightIterable};
use crate::position::{TapOffset};
use crate::iterators::{SignedCompositions, WeakCompositions};


pub trait PoolIterable {
    type Offset: TapOffset;

    fn advance(&mut self) -> bool;
    fn reset(&mut self);
    fn weight_state(&self) -> &[isize];
    fn set_new_sum(&mut self, sum: isize);
}

pub trait NoRotationPoolIterable<'a> : PoolIterable {
    fn new(pool: &'a [Self::Offset], size: usize) -> Self;
    fn state(&self) -> Self::Offset;
}

pub trait RotationPoolIterable<'a> : PoolIterable {
    fn new(pool: &'a [Vec<Self::Offset>], size: usize) -> Self;
    fn state(&self) -> Vec<Self::Offset>;
}

pub struct NoRotationPoolIterator<'a, O: TapOffset, Weights: WeightIterable> {
    pool: &'a [O],
    iterator: Weights,
}

impl<O: TapOffset, Weights: WeightIterable> PoolIterable for NoRotationPoolIterator<'_, O, Weights> {
    type Offset = O;

    fn advance(&mut self)  -> bool {
        self.iterator.advance()
    }

    fn reset(&mut self) {
        self.iterator.reset();
    }

    fn weight_state(&self) -> &[isize] {
        return self.iterator.current()
    }

    fn set_new_sum(&mut self, sum: isize) {
        self.iterator.set_new_sum(sum);
    }
}

impl<'a, O: TapOffset, Weights: WeightIterable> NoRotationPoolIterable<'a> for NoRotationPoolIterator<'a, O, Weights> {
    fn new(pool: &'a [O], count: usize) -> Self {
        let k = pool.len();

        Self {
            pool,
            iterator: Weights::new(count as isize, k),
        }
    }

    fn state(&self) -> Self::Offset  {
        return self.pool.iter().zip(self.iterator.current().iter()).fold(Self::Offset::default(), |acc, (offset, weight)| {acc + *offset * (*weight as f32)});
    }
}

pub struct RotationPoolIterator<'a, O: TapOffset, Weights: WeightIterable> {
    pool: &'a [Vec<O>],
    iterator: Weights,
    len_facings: usize,
}

impl<O: TapOffset, Weights: WeightIterable> PoolIterable for RotationPoolIterator<'_, O, Weights> {
    type Offset = O;

    fn advance(&mut self)  -> bool {
        self.iterator.advance()
    }

    fn reset(&mut self) {
        self.iterator.reset();
    }

    fn weight_state(&self) -> &[isize] {
        self.iterator.current()
    }

    fn set_new_sum(&mut self, sum: isize) {
        self.iterator.set_new_sum(sum);
    }
}

impl<'a, O: TapOffset, Weights: WeightIterable> RotationPoolIterable<'a> for RotationPoolIterator<'a, O, Weights> {
    fn new(pool: &'a [Vec<O>], count: usize) -> Self {
        let k = pool.len();
        let len_facings = pool[0].len();

        Self {
            pool,
            iterator: Weights::new(count as isize, k),
            len_facings,
        }
    }

    fn state(&self) -> Vec<Self::Offset>  {
        let current = self.iterator.current();

        (0..self.len_facings).map(
            |i| current.iter().zip(self.pool.iter()).fold(
                Self::Offset::default(), |acc, (weight, tap)| acc + tap[i] * (*weight as f32)
                )
            ).collect()
    }
}

pub enum NoRotationPool<'a, Offset: TapOffset> {
    Reversible(NoRotationPoolIterator<'a, Offset, SignedCompositions>),
    Irreversible(NoRotationPoolIterator<'a, Offset, WeakCompositions>),
}

impl<O: TapOffset> PoolIterable for NoRotationPool<'_, O> {
    type Offset = O;

    fn advance(&mut self) -> bool {
        match self {
            NoRotationPool::Reversible(it) => it.advance(),
            NoRotationPool::Irreversible(it) => it.advance(),
        }
    }

    fn reset(&mut self) {
        match self {
            NoRotationPool::Reversible(it) => it.reset(),
            NoRotationPool::Irreversible(it) => it.reset(),
        }
    }

    fn weight_state(&self) -> &[isize] {
        match self {
            NoRotationPool::Reversible(it) => it.weight_state(),
            NoRotationPool::Irreversible(it) => it.weight_state(),
        }
    }

    fn set_new_sum(&mut self, sum: isize) {
        match self {
            NoRotationPool::Reversible(it) => it.set_new_sum(sum),
            NoRotationPool::Irreversible(it) => it.set_new_sum(sum),
        }
    }
}

impl<'a, O: TapOffset> NoRotationPoolIterable<'a> for NoRotationPool<'_, O> {
    fn new(_: &'a [Self::Offset], _: usize) -> Self {
        panic!("Not allowed!");
    }

    fn state(&self) -> Self::Offset {
        match self {
            NoRotationPool::Reversible(it) => it.state(),
            NoRotationPool::Irreversible(it) => it.state(),
        }
    }
}

pub enum RotationPool<'a, Offset: TapOffset> {
    Reversible(RotationPoolIterator<'a, Offset, SignedCompositions>),
    Irreversible(RotationPoolIterator<'a, Offset, WeakCompositions>),
}

impl<O: TapOffset> PoolIterable for RotationPool<'_, O> {
    type Offset = O;

    fn advance(&mut self) -> bool {
        match self {
            RotationPool::Reversible(it) => it.advance(),
            RotationPool::Irreversible(it) => it.advance(),
        }
    }

    fn reset(&mut self) {
        match self {
            RotationPool::Reversible(it) => it.reset(),
            RotationPool::Irreversible(it) => it.reset(),
        }
    }

    fn weight_state(&self) -> &[isize] {
        match self {
            RotationPool::Reversible(it) => it.weight_state(),
            RotationPool::Irreversible(it) => it.weight_state(),
        }
    }

    fn set_new_sum(&mut self, sum: isize) {
        match self {
            RotationPool::Reversible(it) => it.set_new_sum(sum),
            RotationPool::Irreversible(it) => it.set_new_sum(sum),
        }
    }
}

impl<'a, O: TapOffset> RotationPoolIterable<'a> for RotationPool<'_, O> {
    fn new(_: &'a [Vec<Self::Offset>], _: usize) -> Self {
        panic!("Not allowed!");
    }

    fn state(&self) -> Vec<Self::Offset> {
        match self {
            RotationPool::Reversible(it) => it.state(),
            RotationPool::Irreversible(it) => it.state(),
        }
    }
}