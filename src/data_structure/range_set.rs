pub mod range_set {
    use std::{collections::BTreeSet, i64};

    pub struct RangeSet {
        bset: BTreeSet<(i64, i64)>,
    }

    #[allow(dead_code)]
    impl RangeSet {
        pub fn new() -> Self {
            Self {
                bset: vec![(i64::MIN, i64::MIN), (i64::MAX, i64::MAX)]
                    .into_iter()
                    .collect(),
            }
        }

        pub fn contains(&self, x: i64) -> bool {
            //! Returns if `x` is contained in the set.

            if let Some(&(l, r)) = self.bset.range(..(x + 1, x + 1)).next_back() {
                return l <= x && x <= r;
            }

            false
        }

        pub fn get_range(&self, x: i64) -> Option<(i64, i64)> {
            //! Returns the range `[l, r)` that covers `x`, or `None` if `x` is not included in any of the ranges.

            if let Some(&(l, r)) = self.bset.range(..(x + 1, x + 1)).next_back() {
                if l <= x && x <= r {
                    return Some((l, r + 1));
                }
            }

            None
        }

        pub fn insert(&mut self, x: i64) {
            //! Inserts `x` into the set. If the set already contained the value, the set is not modified.

            if let (Some(&(pl, pr)), Some(&(nl, nr))) = (
                self.bset.range(..(x + 1, x + 1)).next_back(),
                self.bset.range((x + 1, x + 1)..).next(),
            ) {
                if pl <= x && x <= pr {
                    return;
                }

                if pr == x - 1 {
                    if nl == x + 1 {
                        // concat both sides of `x`
                        self.bset.remove(&(pl, pr));
                        self.bset.remove(&(nl, nr));
                        self.bset.insert((pl, nr));
                    } else {
                        // concat left side of `x`
                        self.bset.remove(&(pl, pr));
                        self.bset.insert((pl, x));
                    }
                } else {
                    if nl == x + 1 {
                        // concat right side of `x`
                        self.bset.remove(&(nl, nr));
                        self.bset.insert((x, nr));
                    } else {
                        self.bset.insert((x, x));
                    }
                }
            }
        }

        pub fn remove(&mut self, x: i64) {
            //! Removes `x` from the set. If `x` is not contained in the set, the set is not modified.

            if let Some(&(l, r)) = self.bset.range(..(x + 1, x + 1)).next_back() {
                if x < l || r < x {
                    return;
                }

                self.bset.remove(&(l, r));

                if l == x {
                    if r == x {
                    } else {
                        self.bset.insert((x + 1, r));
                    }
                } else {
                    if r == x {
                        self.bset.insert((l, x - 1));
                    } else {
                        self.bset.insert((l, x - 1));
                        self.bset.insert((x + 1, r));
                    }
                }
            }
        }

        pub fn mex(&self, x: i64) -> i64 {
            //! Returns the minimum value `y` `(y >= x)` that is not contained in the set.

            if let Some((_, r)) = self.get_range(x) {
                r
            } else {
                x
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::range_set::RangeSet;

    #[test]
    fn test_simple() {
        let mut rset = RangeSet::new();
        assert_eq!(rset.contains(3), false);
        rset.insert(3);
        assert_eq!(rset.contains(3), true);
        rset.remove(3);
        assert_eq!(rset.contains(3), false);
    }

    #[test]
    fn test_consecutive() {
        let mut rset = RangeSet::new();
        rset.insert(3);
        rset.insert(1);
        assert_eq!(rset.contains(2), false);
        rset.insert(2);
        assert_eq!(rset.contains(2), true);
        rset.remove(2);
        assert_eq!(rset.contains(1), true);
        assert_eq!(rset.contains(2), false);
        assert_eq!(rset.contains(3), true);
    }

    #[test]
    fn test_mex() {
        let mut rset = RangeSet::new();
        rset.insert(1);
        assert_eq!(rset.mex(0), 0);
        rset.insert(0);
        assert_eq!(rset.mex(0), 2);
        rset.insert(2);
        assert_eq!(rset.mex(0), 3);
    }
}
