pub mod prefix_sum {
    use std::ops::{Add, Sub};

    pub struct PrefixSum<T> {
        n: usize,
        s: Vec<T>,
    }

    pub struct PrefixSum2D<T> {
        h: usize,
        w: usize,
        s: Vec<Vec<T>>,
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    impl<T> PrefixSum2D<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Copy,
    {
        pub fn new(h: usize, w: usize, e: T, f: impl Fn(usize, usize) -> T) -> Self {
            let mut s = vec![vec![e; w + 1]; h + 1];

            for i in 0..h {
                for j in 0..w {
                    s[i + 1][j + 1] = f(i, j) + s[i][j + 1] + s[i + 1][j] - s[i][j];
                }
            }

            Self { h, w, s }
        }

        pub fn query(&self, lx: usize, rx: usize, ly: usize, ry: usize) -> T {
            //! Returns a sum over `[lx, rx) × [ly, ry)`
            assert!(lx <= rx && ly <= ry);
            assert!(rx <= self.h && ry <= self.w);

            self.s[rx][ry] + self.s[lx][ly] - self.s[lx][ry] - self.s[rx][ly]
        }
    }
}

#[cfg(test)]
mod test {
    use super::prefix_sum::{PrefixSum, PrefixSum2D};

    #[test]
    fn test_1d_simple() {
        let a = vec![1, 3, 5, 7, 9];
        let psum = PrefixSum::new(5, 0, |i| a[i]);

        assert_eq!(psum.query(1, 3), 8);
        assert_eq!(psum.query(2, 5), 21);
        assert_eq!(psum.query(0, 5), 25);
    }

    #[test]
    fn test_2d_simple() {
        let a = vec![vec![1, 2, 3], vec![2, 4, 6], vec![3, 6, 9]];
        let psum = PrefixSum2D::new(3, 3, 0, |i, j| a[i][j]);

        assert_eq!(psum.query(0, 2, 0, 1), 3);
        assert_eq!(psum.query(1, 2, 1, 3), 10);
        assert_eq!(psum.query(0, 3, 0, 3), 36);
    }
}
