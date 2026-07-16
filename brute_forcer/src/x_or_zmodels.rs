use crate::iterators::{WeakCompositions, SignedCompositions};
use crate::models::{Conglomerate, Goal};

trait SingleAxisPoolIterator {
    fn advance(&mut self) -> bool;
    fn reset(&mut self);
    fn state(&self) -> f32;
    fn weight_state(&self) -> &[isize];
}

enum PoolIteratorImpl<'a> {
    Reversible(SingleAxisReversiblePoolIterator<'a>),
    Irreversible(SingleAxisIrreversiblePoolIterator<'a>),
}

impl<'a> SingleAxisPoolIterator for PoolIteratorImpl<'a> {
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

    fn state(&self) -> f32 {
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

struct SingleAxisReversiblePoolIterator<'a> {
    pool: &'a [f32],
    iterator: SignedCompositions,
}

impl<'a> SingleAxisReversiblePoolIterator<'a> {
    fn new(pool: &'a [f32], count: usize) -> Self {
        let k = pool.len();

        Self {
            pool,
            iterator: SignedCompositions::new(count as isize, k),
        }
    }
}

impl SingleAxisPoolIterator for SingleAxisReversiblePoolIterator<'_> {
    fn advance(&mut self) -> bool {
        self.iterator.advance()
    }

    fn reset(&mut self) {
        self.iterator.reset();
    }

    fn state(&self) -> f32 {
        return self.pool.iter().zip(self.iterator.current().iter()).fold(0.0f32, |ax, (x, w)| {ax + x * (*w as f32)});
    }

    fn weight_state(&self) -> &[isize] {
        return self.iterator.current()
    }
}

struct SingleAxisIrreversiblePoolIterator<'a> {
    pool: &'a [f32],
    iterator: WeakCompositions,
}

impl<'a> SingleAxisIrreversiblePoolIterator<'a> {
    fn new(pool: &'a [f32], count: usize) -> Self {
        let k = pool.len();

        Self {
            pool,
            iterator: WeakCompositions::new(count as isize, k),
        }
    }
}

impl SingleAxisPoolIterator for SingleAxisIrreversiblePoolIterator<'_> {
    fn advance(&mut self) -> bool {
        self.iterator.advance()
    }

    fn reset(&mut self) {
        self.iterator.reset();
    }

    fn state(&self) -> f32 {
        return self.pool.iter().zip(self.iterator.current().iter()).fold(0.0f32, |ax, (x, w)| {ax + x * (*w as f32)});
    }

    fn weight_state(&self) -> &[isize] {
        return self.iterator.current()
    }
}


pub struct SingleAxisPoolIteratorConglomerate<'a> {
    iterators: Vec<PoolIteratorImpl<'a>>,
    count_of_pools: usize,
    pivot: usize,
}

impl<'a> SingleAxisPoolIteratorConglomerate<'a> {
    pub fn new(max_counts: Vec<usize>, pools: &'a [Vec<f32>], is_reversible: &[bool]) -> Self {
        let count_of_pools = pools.len();

        let iterators: Vec<PoolIteratorImpl> =
            pools.iter()
                .zip(max_counts.iter())
                .zip(is_reversible.iter())
                .map(|((pool, &count), &rev)| {
                    if rev {
                        PoolIteratorImpl::Reversible(SingleAxisReversiblePoolIterator::new(pool, count))
                    } else {
                        PoolIteratorImpl::Irreversible(SingleAxisIrreversiblePoolIterator::new(pool, count))
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

impl<'a> Conglomerate for SingleAxisPoolIteratorConglomerate<'a> {
    type Pos = f32;

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
        return self.iterators.iter().map(|i| i.state()).sum();
    }

    fn weight_state(&self) -> Vec<&[isize]> {
        return self.iterators.iter().map(|i| i.weight_state()).collect()
    }
}

pub struct SingleAxisGoal {
    start: f32,
    end: f32,
    span: f32,
    center: f32,
}

impl SingleAxisGoal {
    pub fn new(start: f32, end: f32) -> Self {
        let center: f32;
        if start == f32::INFINITY || start == f32::NEG_INFINITY{
            center = start
        } else if end == f32::INFINITY || end == f32::NEG_INFINITY {
            center = end
        } else {
            center = (start + end)/2.0f32
        }

        let span: f32 = end - start;

        Self { start, end, span, center }
    }
}

impl Goal<f32> for SingleAxisGoal {
    type Dist = (f32, f32);

    fn is_satisfied(&self, point: f32) -> bool {
        return self.start < point && point < self.end
    }

    fn get_quality(&self, point: f32) -> f32 {
        return ((point - self.center)/self.span).abs()
    }

    fn get_dists(&self, point: f32) -> (f32, f32) {
        return ((point - self.start).abs(), (point - self.end).abs())
    }
}

pub struct AxisProjectedGoal { inner: SingleAxisGoal, axis: usize }

impl AxisProjectedGoal {
    pub fn new(start: f32, end: f32, axis: usize) -> Self {
        Self { 
            inner: SingleAxisGoal::new(start, end),
            axis,
        }
    }
}

impl Goal<(f32, f32)> for AxisProjectedGoal {
    type Dist = (f32, f32);
    fn is_satisfied(&self, pos: (f32, f32)) -> bool {
        self.inner.is_satisfied(if self.axis == 0 { pos.0 } else { pos.1 })
    }

    fn get_quality(&self, pos: (f32, f32)) -> f32 {
        self.inner.get_quality(if self.axis == 0 { pos.0 } else { pos.1 })
    }

    fn get_dists(&self, pos: (f32, f32)) -> (f32, f32) {
        self.inner.get_dists(if self.axis == 0 { pos.0 } else { pos.1 })
    }
}