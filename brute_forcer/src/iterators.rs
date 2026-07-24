

pub struct MaxCountIter {
    max_counts: Vec<usize>,
    n: usize,
    item: Vec<usize>,
    next: Vec<usize>,
    sum: usize,
    depth: usize,
    finished: bool,
}

impl MaxCountIter {
    pub fn new(max_counts: Vec<usize>, n: usize) -> Self {
        let len = max_counts.len();
        Self {
            max_counts,
            n,
            item: vec![0; len],
            next: vec![0; len],
            sum: 0,
            depth: 0,
            finished: false,
        }
    }

    fn upper_bound(&self, k: usize) -> usize {
        if self.max_counts[k] != 0 {
            self.max_counts[k].min(self.n - self.sum)
        } else {
            assert!(k != 0);
            let prev = self.item[k - 1];
            self.max_counts[k - 1]
                .saturating_sub(prev)
                .min(self.n - self.sum)
        }
    }
}

impl Iterator for MaxCountIter {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let m = self.item.len();

        loop {
            if self.depth == m {
                let result = self.item.clone();

                // backtrack
                self.depth -= 1;
                self.sum -= self.item[self.depth];
                self.next[self.depth] = self.item[self.depth] + 1;

                return Some(result);
            }

            let ub = self.upper_bound(self.depth);

            if self.next[self.depth] <= ub {
                let v = self.next[self.depth];

                self.item[self.depth] = v;
                self.sum += v;

                self.depth += 1;

                if self.depth < m {
                    self.next[self.depth] = 0;
                }
            } else {
                if self.depth == 0 {
                    self.finished = true;
                    return None;
                }

                self.next[self.depth] = 0;

                self.depth -= 1;
                self.sum -= self.item[self.depth];
                self.next[self.depth] = self.item[self.depth] + 1;
            }
        }
    }
}

pub struct WeakCompositions {
    current: Vec<isize>,
    finished: bool,
    parts: usize, 
    sum: isize,
}

impl WeakCompositions {
    pub fn new(sum: isize, parts: usize) -> Self {
        assert!(parts > 0);

        let mut current: Vec<isize> = vec![0; parts];
        current[0] = sum;

        Self {
            current,
            finished: false,
            parts,
            sum,
        }
    }
}

impl WeightIterable for WeakCompositions {
    fn advance(&mut self) -> bool {
        let k = self.parts;

        if k == 1 {
            return true;
        }

        let pivot = (0..k - 1)
            .rev()
            .find(|&i| self.current[i] > 0);

        match pivot {
            None => {
                self.finished = true;
            }
            Some(i) => {
                self.current[i] -= 1;
                self.current[i + 1] += 1;

                if i + 1 < k - 1 {
                    let remainder = self.current[k - 1];
                    self.current[i + 1] += remainder;
                    self.current[k - 1] = 0;
                }
            }
        }

        if self.finished {
            return true;
        } else {
            return false
        }
    }
    
    fn reset(&mut self) {
        let mut start: Vec<isize> = vec![0; self.parts];
        start[0] = self.sum;

        self.current = start;
        self.finished = false;
    }

    fn current(&self) -> &[isize] {
        &self.current
    }

    fn set_new_sum(&mut self, sum: isize) {
        self.sum = sum;
        self.reset();
    }
}

pub struct SignedCompositions {
    base: WeakCompositions,
    pivot: usize,
}

impl SignedCompositions {
    pub fn new(sum: isize, k: usize) -> Self {
        Self {
            base: WeakCompositions::new(sum, k),
            pivot: 0,
        }
    }

    fn advance_sign(&mut self) -> bool {
        loop {
            if self.base.current[self.pivot] > 0 {
                self.base.current[self.pivot] *= -1;
                self.pivot = 0;

                return false
            } else if self.base.current[self.pivot] <= 0 {
                self.base.current[self.pivot] *= -1;
                self.pivot += 1;

                if self.pivot >= self.base.parts {
                    self.pivot = 0;
                    return true
                }
            }
        }
    }
}

impl WeightIterable for SignedCompositions {
    fn advance(&mut self) -> bool {
        if !self.advance_sign() {
            return false
        } else if !self.base.advance() {
            return false
        } else {
            return true
        }
    }

    fn reset(&mut self) {
        self.base.reset();
        self.pivot = 0;
    }

    fn current(&self) -> &[isize] {
        return self.base.current()
    }

    fn set_new_sum(&mut self, sum: isize) {
        self.base.sum = sum;
        self.reset();
    }
}

pub trait WeightIterable {
    fn advance(&mut self) -> bool;
    fn reset(&mut self);
    fn current(&self) -> &[isize];
    fn set_new_sum(&mut self, sum: isize);
}

pub enum WeightIterator {
    Reversible(SignedCompositions),
    Irreversible(WeakCompositions),
}

impl WeightIterable for WeightIterator {
    fn advance(&mut self) -> bool {
        match self {
            WeightIterator::Reversible(it) => it.advance(),
            WeightIterator::Irreversible(it) => it.advance(),
        }
    }

    fn reset(&mut self) {
        match self {
            WeightIterator::Reversible(it) => it.reset(),
            WeightIterator::Irreversible(it) => it.reset(),
        }
    }

    fn current(&self) -> &[isize] {
        match self {
            WeightIterator::Reversible(it) => it.current(),
            WeightIterator::Irreversible(it) => it.current(),
        }
    }

    fn set_new_sum(&mut self, sum: isize) {
        match self {
            WeightIterator::Reversible(it) => it.set_new_sum(sum),
            WeightIterator::Irreversible(it) => it.set_new_sum(sum),
        }
    }
}