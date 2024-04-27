pub mod segment_tree {
    pub struct SegmentTree<T, F> {
        id: T,
        op: F,
        size: usize,
        values: Vec<T>,
    }

    #[allow(dead_code)]
    impl<T: Copy, F: Fn(T, T) -> T> SegmentTree<T, F> {
        pub fn new(n: usize, id: T, op: F) -> Self {
            //! Constructs a segment tree `a[i]` for array size `n`.
            SegmentTree {
                id: id,
                op: op,
                size: n,
                values: vec![id; 2 * n],
            }
        }

        pub fn construct(&mut self, seq: &Vec<T>) {
            //! Constructs a segment tree `a[i]` from a vector `seq`.
            assert!(seq.len() == self.size);

            for (i, &x) in seq.iter().enumerate() {
                self.values[i + self.size] = x;
            }

            for i in (1..self.size).rev() {
                self.values[i] = (self.op)(self.values[i << 1], self.values[i << 1 | 1]);
            }
        }

        pub fn update(&mut self, mut i: usize, x: T) {
            //! Makes a point update: `a[i] <- x`.
            assert!(i < self.size);

            i += self.size;
            self.values[i] = x;
            while i > 1 {
                i >>= 1;
                self.values[i] = (self.op)(self.values[i << 1], self.values[i << 1 | 1]);
            }
        }

        pub fn prod(&self, mut l: usize, mut r: usize) -> T {
            //! Returns a product over the interval `[l, r)`.
            assert!(l <= r && r <= self.size);

            l += self.size;
            r += self.size;

            let mut prod_l = self.id;
            let mut prod_r = self.id;

            while l < r {
                if l & 1 == 1 {
                    prod_l = (self.op)(prod_l, self.values[l]);
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    prod_r = (self.op)(self.values[r], prod_r);
                }

                l >>= 1;
                r >>= 1;
            }

            (self.op)(prod_l, prod_r)
        }

        pub fn get(&self, i: usize) -> T {
            //! Returns a value of `a[i]`.
            assert!(i < self.size);

            self.values[i + self.size]
        }
    }
}
