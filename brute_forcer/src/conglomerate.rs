
use crate::iterators::{SignedCompositions, WeakCompositions};
use crate::position::{TapOffset};
use crate::models::{NoRotationPool, RotationPool, PoolIterable, NoRotationPoolIterable, RotationPoolIterable, NoRotationPoolIterator, RotationPoolIterator};

pub trait Conglomerate {
    type Offset: TapOffset;
    fn weight_state(&self) -> Vec<&[isize]>;
    fn advance(&mut self) -> bool;
    fn update_max_counts(&mut self, max_counts: &Vec<usize>);
}

pub trait NoRotationConglomerate: Conglomerate {
    fn offset(&self) -> Self::Offset;
}

pub trait RotationConglomerate: Conglomerate {
    fn offset(&self) -> Vec<Self::Offset>;
}


pub struct NoRotationPoolIteratorConglomerate<'a, O: TapOffset> {
    iterators: Vec<NoRotationPool<'a, O>>,
    pools: &'a [Vec<O>],
    count_of_pools: usize,
    pivot: usize,
}

impl<'a, O: TapOffset> NoRotationPoolIteratorConglomerate<'a, O> {
    pub fn new(pools: &'a Vec<Vec<O>>, is_reversible: &[bool]) -> Self {
        let count_of_pools = pools.len();
        let pool_sizes: Vec<usize> = pools.iter().map(|pool| pool.len()).collect();
        let iterators: Vec<NoRotationPool<O>> =
            is_reversible.into_iter().zip(pool_sizes.into_iter()).zip(pools.iter())
                .map(|((&rev, size), pool)| {
                    if rev {
                        let pool = NoRotationPoolIterator::<'_, O, SignedCompositions>::new(pool, size);
                        NoRotationPool::Reversible(pool)
                    } else {
                        let pool = NoRotationPoolIterator::<'_, O, WeakCompositions>::new(pool, size);
                        NoRotationPool::Irreversible(pool)
                    }
                })
                .collect();

        NoRotationPoolIteratorConglomerate { iterators, pools, count_of_pools, pivot: 0 }
    }
}

impl<'a, O: TapOffset> Conglomerate for NoRotationPoolIteratorConglomerate<'a, O> {
    type Offset = O;

    fn weight_state(&self) -> Vec<&[isize]> {
        self.iterators.iter().map(|iter| iter.weight_state()).collect()
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

    fn update_max_counts(&mut self, max_counts: &Vec<usize>) {
        self.pivot = 0;

        for (iterator, new_max_count) in self.iterators.iter_mut().zip(max_counts) {
            iterator.set_new_sum(*new_max_count as isize);
        }
    }
}

impl<'a, O: TapOffset> NoRotationConglomerate for NoRotationPoolIteratorConglomerate<'a, O> {
    fn offset(&self) -> O {
        self.iterators.iter().zip(self.pools)
            .map(
                |(iter, pool)|
                    iter.weight_state().iter().zip(pool).fold(
                        O::default(), |acc, (weight, offset)| acc + *offset * (*weight as f32)
                    )
                )
            .fold(O::default(), |acc, offset| acc + offset)
    }
}

pub struct RotationPoolIteratorConglomerate<'a, O: TapOffset> {
    iterators: Vec<RotationPool<'a, O>>,
    pools: &'a [Vec<Vec<O>>],
    count_of_pools: usize,
    pivot: usize,
    len_facings: usize,
}

impl<'a, O: TapOffset> RotationPoolIteratorConglomerate<'a, O> {
    pub fn new(pools: &'a Vec<Vec<Vec<O>>>, is_reversible: &[bool]) -> Self {
        let len_facings = pools[0][0].len();
        let count_of_pools = pools.len();
        let pool_sizes: Vec<usize> = pools.iter().map(|pool| pool.len()).collect();
        let iterators: Vec<RotationPool<O>> =
            is_reversible.into_iter().zip(pool_sizes.into_iter()).zip(pools.iter())
                .map(|((&rev, size), pool)| {
                    if rev {
                        let pool = RotationPoolIterator::<'_, O, SignedCompositions>::new(pool, size);
                        RotationPool::Reversible(pool)
                    } else {
                        let pool = RotationPoolIterator::<'_, O, WeakCompositions>::new(pool, size);
                        RotationPool::Irreversible(pool)
                    }
                })
                .collect();

        RotationPoolIteratorConglomerate { iterators, pools, count_of_pools, pivot: 0, len_facings }
    }
}

impl<'a, O: TapOffset> Conglomerate for RotationPoolIteratorConglomerate<'a, O> {
    type Offset = O;

    fn weight_state(&self) -> Vec<&[isize]> {
        self.iterators.iter().map(|iter| iter.weight_state()).collect()
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

    fn update_max_counts(&mut self, max_counts: &Vec<usize>) {
        self.pivot = 0;

        for (iterator, new_max_count) in self.iterators.iter_mut().zip(max_counts) {
            iterator.set_new_sum(*new_max_count as isize);
        }
    }
}

impl<'a, O: TapOffset> RotationConglomerate for RotationPoolIteratorConglomerate<'a, O> {
    fn offset(&self) -> Vec<O> {
        let states: Vec<Vec<O>> = self.iterators.iter().map(|iterator| iterator.state()).collect();
        (0..self.len_facings).map(
            |i| states.iter().fold(
                O::default(), |acc, state| acc + state[i]
                )
            ).collect()
    }
}