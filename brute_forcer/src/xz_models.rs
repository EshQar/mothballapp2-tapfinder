use crate::iterators::{WeakCompositions, SignedCompositions};
use crate::models::{Conglomerate, Goal};

trait PoolIterator {
    fn advance(&mut self) -> bool;
    fn reset(&mut self);
    fn state(&self) -> (f32, f32);
    fn weight_state(&self) -> &[isize];
}

enum PoolIteratorImpl<'a> {
    Reversible(ReversiblePoolIterator<'a>),
    Irreversible(IrreversiblePoolIterator<'a>),
}

impl<'a> PoolIterator for PoolIteratorImpl<'a> {
    fn advance(&mut self) -> bool {
        match self {
            PoolIteratorImpl::Reversible(it) => it.advance(),
            PoolIteratorImpl::Irreversible(it) => it.advance(),
        }
    }

    fn reset(&mut self) {
        match self {
            PoolIteratorImpl::Reversible(it) => it.reset(),
            PoolIteratorImpl::Irreversible(it) => it.reset(),
        }
    }

    fn state(&self) -> (f32, f32) {
        match self {
            PoolIteratorImpl::Reversible(it) => it.state(),
            PoolIteratorImpl::Irreversible(it) => it.state(),
        }
    }

    fn weight_state(&self) -> &[isize] {
        match self {
            PoolIteratorImpl::Reversible(it) => it.weight_state(),
            PoolIteratorImpl::Irreversible(it) => it.weight_state(),
        }
    }
}

struct ReversiblePoolIterator<'a> {
    pool: &'a [(f32,f32)],
    iterator: SignedCompositions,
}

impl<'a> ReversiblePoolIterator<'a> {
    fn new(pool: &'a [(f32,f32)], count: usize) -> Self {
        let k = pool.len();

        Self {
            pool,
            iterator: SignedCompositions::new(count as isize, k),
        }
    }
}

impl PoolIterator for ReversiblePoolIterator<'_> {
    fn advance(&mut self) -> bool {
        self.iterator.advance()
    }

    fn reset(&mut self) {
        self.iterator.reset();
    }

    fn state(&self) -> (f32, f32) {
        return self.pool.iter().zip(self.iterator.current().iter()).fold((0.0f32, 0.0f32), |(ax, ay), ((x, y), w)| {(ax + x * (*w as f32), ay + y * (*w as f32))});
    }

    fn weight_state(&self) -> &[isize] {
        return self.iterator.current()
    }
}

struct IrreversiblePoolIterator<'a> {
    pool: &'a [(f32,f32)],
    iterator: WeakCompositions,
}

impl<'a> IrreversiblePoolIterator<'a> {
    fn new(pool: &'a [(f32,f32)], count: usize) -> Self {
        let k = pool.len();

        Self {
            pool,
            iterator: WeakCompositions::new(count as isize, k),
        }
    }
}

impl PoolIterator for IrreversiblePoolIterator<'_> {
    fn advance(&mut self) -> bool {
        self.iterator.advance()
    }

    fn reset(&mut self) {
        self.iterator.reset();
    }

    fn state(&self) -> (f32, f32) {
        return self.pool.iter().zip(self.iterator.current().iter()).fold((0.0f32, 0.0f32), |(ax, ay), ((x, y), w)| {(ax + x * (*w as f32), ay + y * (*w as f32))});
    }

    fn weight_state(&self) -> &[isize] {
        return self.iterator.current()
    }
}

trait RotationPoolIterator {
    fn advance(&mut self) -> bool;
    fn reset(&mut self);
    fn state(&self, facing_index: usize) -> (f32, f32);
    fn weight_state(&self) -> &[isize];
}

enum RotationPoolIteratorImpl<'a> {
    Reversible(RotationReversiblePoolIterator<'a>),
    Irreversible(RotationIrreversiblePoolIterator<'a>),
}

impl<'a> RotationPoolIterator for RotationPoolIteratorImpl<'a> {
    fn advance(&mut self) -> bool {
        match self {
            RotationPoolIteratorImpl::Reversible(it) => it.advance(),
            RotationPoolIteratorImpl::Irreversible(it) => it.advance(),
        }
    }

    fn reset(&mut self) {
        match self {
            RotationPoolIteratorImpl::Reversible(it) => it.reset(),
            RotationPoolIteratorImpl::Irreversible(it) => it.reset(),
        }
    }

    fn state(&self, facing_index: usize) -> (f32, f32) {
        match self {
            RotationPoolIteratorImpl::Reversible(it) => it.state(facing_index),
            RotationPoolIteratorImpl::Irreversible(it) => it.state(facing_index),
        }
    }

    fn weight_state(&self) -> &[isize] {
        match self {
            RotationPoolIteratorImpl::Reversible(it) => it.weight_state(),
           RotationPoolIteratorImpl::Irreversible(it) => it.weight_state(),
        }
    }
}


struct RotationReversiblePoolIterator<'a> {
    pool: &'a [Vec<(f32, f32)>],
    iterator: SignedCompositions,
}

impl<'a> RotationReversiblePoolIterator<'a> {
    fn new(pool: &'a [Vec<(f32, f32)>], count: usize) -> Self {
        let k = pool.len();

        Self {
            pool,
            iterator: SignedCompositions::new(count as isize, k),
        }
    }
}

impl RotationPoolIterator for RotationReversiblePoolIterator<'_> {
    fn advance(&mut self) -> bool {
        self.iterator.advance()
    }

    fn reset(&mut self) {
        self.iterator.reset();
    }

    fn state(&self, facing_index: usize) -> (f32, f32) {
        return self.pool.iter().zip(self.iterator.current().iter()).fold((0.0f32, 0.0f32), |(ax, ay), (states, w)| { let (x, y) = states[facing_index]; (ax + x * (*w as f32), ay + y * (*w as f32))});
    }

    fn weight_state(&self) -> &[isize] {
        return self.iterator.current()
    }
}

struct RotationIrreversiblePoolIterator<'a> {
    pool: &'a [Vec<(f32, f32)>],
    iterator: WeakCompositions,
}

impl<'a> RotationIrreversiblePoolIterator<'a> {
    fn new(pool: &'a [Vec<(f32, f32)>], count: usize) -> Self {
        let k = pool.len();

        Self {
            pool,
            iterator: WeakCompositions::new(count as isize, k),
        }
    }
}

impl RotationPoolIterator for RotationIrreversiblePoolIterator<'_> {
    fn advance(&mut self) -> bool {
        self.iterator.advance()
    }

    fn reset(&mut self) {
        self.iterator.reset();
    }

    fn state(&self, facing_index: usize) -> (f32, f32) {
        return self.pool.iter().zip(self.iterator.current().iter()).fold((0.0f32, 0.0f32), |(ax, ay), (states, w)| { let (x, y) = states[facing_index]; (ax + x * (*w as f32), ay + y * (*w as f32))});
    }

    fn weight_state(&self) -> &[isize] {
        return self.iterator.current()
    }
}

pub struct PoolIteratorConglomerate<'a> {
    iterators: Vec<PoolIteratorImpl<'a>>,
    count_of_pools: usize,
    pivot: usize,
}

impl<'a> PoolIteratorConglomerate<'a> {
    pub fn new(max_counts: Vec<usize>, pools: &'a [Vec<(f32, f32)>], is_reversible: &[bool]) -> Self {
        let count_of_pools = pools.len();

        let iterators: Vec<PoolIteratorImpl> =
            pools.iter()
                .zip(max_counts.iter())
                .zip(is_reversible.iter())
                .map(|((pool, &count), &rev)| {
                    if rev {
                        PoolIteratorImpl::Reversible(ReversiblePoolIterator::new(pool, count))
                    } else {
                        PoolIteratorImpl::Irreversible(IrreversiblePoolIterator::new(pool, count))
                    }
                })
                .collect();

        Self {
            iterators,
            count_of_pools,
            pivot: 0,
        }
    }
}

impl<'a> Conglomerate for PoolIteratorConglomerate<'a> {
    type Pos = (f32, f32);

    fn advance(&mut self) -> bool {
        if self.iterators[self.pivot].advance() {

            self.iterators[self.pivot].reset();
            self.pivot += 1;

            if self.pivot >= self.count_of_pools {
                return true
            }

            return self.advance()
        } else {
            self.pivot = 0;
            return false
        }
    }

    fn pos(&self) -> Self::Pos {
        return self.iterators.iter().map(|i| i.state()).fold((0.0f32, 0.0f32), |(ax, ay), (x, y)| {(ax + x, ay + y)});
    }

    fn weight_state(&self) -> Vec<&[isize]> {
        return self.iterators.iter().map(|i| i.weight_state()).collect()
    }
}

pub struct RotationPoolIteratorConglomerate<'a> {
    iterators: Vec<RotationPoolIteratorImpl<'a>>,
    count_of_pools: usize,
    pivot: usize,
    pub facing_index: usize,
}

impl<'a> RotationPoolIteratorConglomerate<'a> {
    pub fn new(max_counts: Vec<usize>, pools: &'a Vec<Vec<Vec<(f32, f32)>>>, is_reversible: &[bool]) -> Self {
        let count_of_pools = pools.len();

        let iterators: Vec<RotationPoolIteratorImpl> =
            pools.iter()
                .zip(max_counts.iter())
                .zip(is_reversible.iter())
                .map(|((pool, &count), &rev)| {
                    if rev {
                        RotationPoolIteratorImpl::Reversible(RotationReversiblePoolIterator::new(pool, count))
                    } else {
                        RotationPoolIteratorImpl::Irreversible(RotationIrreversiblePoolIterator::new(pool, count))
                    }
                })
                .collect();

        Self {
            iterators,
            count_of_pools,
            pivot: 0,
            facing_index: 0,
        }
    }
}

impl<'a> Conglomerate for RotationPoolIteratorConglomerate<'a> {
    type Pos = (f32, f32);

    fn advance(&mut self) -> bool {
        if self.iterators[self.pivot].advance() {

            self.iterators[self.pivot].reset();
            self.pivot += 1;

            if self.pivot >= self.count_of_pools {
                return true
            }

            return self.advance()
        } else {
            self.pivot = 0;
            return false
        }
    }

    fn pos(&self) -> Self::Pos {
        return self.iterators.iter().map(|i| i.state(self.facing_index)).fold((0.0f32, 0.0f32), |(ax, ay), (x, y)| {(ax + x, ay + y)})
    }

    fn weight_state(&self) -> Vec<&[isize]> {
        return self.iterators.iter().map(|i| i.weight_state()).collect()
    }
}

pub struct XZGoal {
    start: (f32, f32),
    end: (f32, f32),
    span: (f32, f32),
    center: (f32, f32),
}

impl XZGoal {
    pub fn new(start: (f32, f32), end: (f32, f32)) -> Self {
        let center: (f32, f32);
        if start.0 == f32::INFINITY || start.0 == f32::NEG_INFINITY {
            center = (end.0, (start.1 + end.1)/2.0f32);
        } else if end.0 == f32::INFINITY || end.0 == f32::NEG_INFINITY {
            center = (start.0, (start.1 + end.1)/2.0f32)
        } else if start.1 == f32::INFINITY || start.1 == f32::NEG_INFINITY {
            center = ((start.0 + end.0)/2.0f32, end.1)
        } else if end.1 == f32::INFINITY || end.1 == f32::NEG_INFINITY {
            center = ((start.0 + end.0)/2.0f32, start.1)
        } else {
            center = ((start.0 + end.0)/2.0f32, (start.1 + end.1)/2.0f32)
        }

        let span = (start.0 - end.0, start.1 - end.1);

        Self { start, end, span, center }
    }
}

impl Goal<(f32, f32)> for XZGoal {
    type Dist = (f32, f32, f32, f32);

    fn is_satisfied(&self, point: (f32, f32)) -> bool {
        return (self.start.0 < point.0 && point.0 < self.end.0) && (self.start.1 < point.1 && point.1 < self.end.1)
    }

    fn get_quality(&self, point: (f32, f32)) -> f32 {
        return ((point.0 - self.center.0)/self.span.0).abs() + ((point.1 - self.center.1)/self.span.1).abs()
    }

    fn get_dists(&self, point: (f32, f32)) -> (f32, f32, f32, f32) {
        return ((point.0 - self.start.0).abs(), (point.1 - self.start.1).abs(), (point.0 - self.end.0).abs(), (point.1 - self.end.1).abs())
    }
}