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

    /// Decrement counts from an iterator.
    pub fn subtract<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            *self.counts.entry(item).or_insert(0) -= 1;
        }
    }

    /// Get count (returns 0 if missing).
    pub fn get(&self, item: &T) -> isize {
        *self.counts.get(item).unwrap_or(&0)
    }

    /// Set a count.
    pub fn set(&mut self, item: T, count: isize) {
        self.counts.insert(item, count);
    }

    /// Remove entries whose count <= 0.
    pub fn prune(&mut self) {
        self.counts.retain(|_, v| *v > 0);
    }

    /// Total of all counts.
    pub fn total(&self) -> isize {
        self.counts.values().sum()
    }

    /// Number of distinct elements.
    pub fn len(&self) -> usize {
        self.counts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }

    /// Most common elements.
    pub fn most_common(&self) -> Vec<(&T, isize)> {
        let mut v: Vec<_> = self
            .counts
            .iter()
            .map(|(k, &c)| (k, c))
            .collect();

        v.sort_by(|a, b| b.1.cmp(&a.1));
        v
    }

    /// Iterator over repeated elements (like Counter.elements()).
    pub fn elements(&self) -> impl Iterator<Item = &T> {
        self.counts
            .iter()
            .filter(|(_, c)| **c > 0)
            .flat_map(|(k, c)| std::iter::repeat_n(k, *c as usize))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&T, &isize)> {
        self.counts.iter()
    }

    pub fn keys(&self) -> impl Iterator<Item = &T> {
        self.counts.keys()
    }
}