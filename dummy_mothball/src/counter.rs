use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Default)]
pub struct Counter<T> {
    counts: HashMap<T, isize>,
}

impl<T: Eq + Hash> Counter<T> {
    pub fn new() -> Self {
        Self {
            counts: HashMap::new(),
        }
    }

    /// Count items from an iterator.
    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut counter = Self::new();
        counter.update(iter);
        counter
    }

    /// Increment counts from an iterator.
    pub fn update<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            *self.counts.entry(item).or_insert(0) += 1;
        }
    }

    /// Get count (returns 0 if missing).
    pub fn get(&self, item: &T) -> isize {
        *self.counts.get(item).unwrap_or(&0)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &isize)> {
        self.counts.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &T> {
        self.counts.keys()
    }
}