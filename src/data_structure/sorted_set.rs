pub mod sorted_set {
    use crate::utils::binary_search::binary_search::lower_bound;

    pub struct SortedSet<T> {
        size: usize,
        values: Vec<Vec<T>>,
    }

    impl<T: PartialOrd + Clone> SortedSet<T> {
        const SPLIT_RATIO: usize = 24;

        pub fn new() -> Self {
            Self {
                size: 0,
                values: vec![],
            }
        }

        fn _position(&self, x: &T) -> (usize, usize) {
            assert!(self.size > 0);

            for (i, bucket) in self.values.iter().enumerate() {
                if *x <= bucket[bucket.len() - 1] {
                    return (i, lower_bound(bucket, x.clone()));
                }
            }

            (
                self.values.len() - 1,
                lower_bound(&self.values[self.values.len() - 1], x.clone()),
            )
        }

        pub fn contains(&mut self, x: T) -> bool {
            //! Returns if `x` is contained in the set.

            if self.size == 0 {
                return false;
            }

            let (bidx, i) = self._position(&x);

            i != self.values[bidx].len() && self.values[bidx][i] == x
        }

        pub fn insert(&mut self, x: T) -> bool {
            //! Inserts `x` into the set.

            if self.size == 0 {
                self.values.push(vec![x]);
                self.size = 1;
                return true;
            }

            let (bidx, i) = self._position(&x);

            if i != self.values[bidx].len() && self.values[bidx][i] == x {
                return false;
            }

            self.values[bidx].insert(i, x);
            self.size += 1;

            if self.values[bidx].len() > self.values.len() * Self::SPLIT_RATIO {
                let mid = self.values[bidx].len() >> 1;
                let (fi, se) = (
                    self.values[bidx][..mid].to_vec(),
                    self.values[bidx][mid..].to_vec(),
                );
                self.values[bidx] = fi;
                self.values.insert(bidx + 1, se);
            }

            true
        }

        pub fn remove(&mut self, x: T) -> bool {
            //! Removes `x` from the set.

            if self.size == 0 {
                return false;
            }

            let (bidx, i) = self._position(&x);

            if i == self.values[bidx].len() || self.values[bidx][i] != x {
                return false;
            }

            self.values[bidx].remove(i);
            self.size -= 1;
            if self.values[bidx].is_empty() {
                self.values.remove(bidx);
            }

            true
        }

        pub fn kth_element(&self, mut k: usize) -> Option<T> {
            //! Returns the `k`-th smallest element.

            for bucket in &self.values {
                if k < bucket.len() {
                    return Some(bucket[k].clone());
                }
                k -= bucket.len();
            }

            None
        }

        pub fn all_elements(&self) -> Vec<&T> {
            //! Returns all elements in ascending order.

            let mut ret = vec![];
            for bucket in &self.values {
                for x in bucket {
                    ret.push(x);
                }
            }

            ret
        }
    }
}

#[cfg(test)]
mod test {
    use super::sorted_set::SortedSet;

    #[test]
    fn test_simple() {
        let mut s = SortedSet::new();
        s.insert(3);
        s.insert(7);
        s.insert(2);
        s.insert(5);
        assert_eq!(s.all_elements(), vec![&2, &3, &5, &7]);
        assert_eq!(s.kth_element(1), Some(3));
        assert_eq!(s.kth_element(4), None);
        assert_eq!(s.contains(3), true);
        s.remove(3);
        assert_eq!(s.kth_element(1), Some(5));
        assert_eq!(s.contains(3), false);
    }
}
