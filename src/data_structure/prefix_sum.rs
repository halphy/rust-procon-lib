pub mod prefix_sum {
    use std::ops::{Add, Sub};

    pub struct PrefixSum<T> {
        n: usize,
        s: Vec<T>,
    }

    impl<T> PrefixSum<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy,
    {
        pub fn new(n: usize, e: T, f: impl Fn(usize) -> T) -> Self {
            let mut s = vec![e; n + 1];

            for i in 0..n {
                s[i + 1] = s[i] + f(i);
            }

            Self { n, s }
        }

        pub fn query(&self, l: usize, r: usize) -> T {
            //! Returns a sum over `[l, r)`.
            assert!(l <= r && r <= self.n);

            self.s[r] - self.s[l]
        }
    }
}

#[cfg(test)]
mod test {
    use super::prefix_sum::PrefixSum;

    #[test]
    fn test_1d_simple() {
        let a = vec![1, 3, 5, 7, 9];
        let psum = PrefixSum::new(5, 0, |i| a[i]);

        assert_eq!(psum.query(1, 3), 8);
        assert_eq!(psum.query(2, 5), 21);
        assert_eq!(psum.query(0, 5), 25);
    }
}
