pub mod binomial {
    use ac_library::modint::ModIntBase;

    pub struct BinomialCoefficient<T> {
        factorial: Vec<T>,
        factorial_inv: Vec<T>,
    }

    impl<T: ModIntBase> BinomialCoefficient<T> {
        pub fn new(size: usize) -> Self {
            let zero = T::new(0);
            let one = T::new(1);

            let mut factorial = vec![zero; size];
            let mut factorial_inv = vec![zero; size];
            let mut inv = vec![zero; size];

            (factorial[0], factorial[1]) = (one, one);
            (factorial_inv[0], factorial_inv[1]) = (one, one);
            (inv[0], inv[1]) = (zero, one);

            for i in 2..size {
                factorial[i] = factorial[i - 1] * T::new(i);
                inv[i] = -inv[T::modulus() as usize % i] * T::new(T::modulus() as usize / i);
                factorial_inv[i] = factorial_inv[i - 1] * inv[i];
            }

            Self {
                factorial,
                factorial_inv,
            }
        }

        pub fn factorial(&self, n: usize) -> T {
            //! Returns the factorial `n!`.

            assert!(n < self.factorial.len());

            self.factorial[n]
        }

        pub fn binomial_coefficient(&self, n: usize, r: usize) -> T {
            //! Returns the binomial coefficient `C(n, r)`.

            assert!(n < self.factorial.len());

            if n < r {
                return T::new(0);
            }

            self.factorial[n] * self.factorial_inv[r] * self.factorial_inv[n - r]
        }
    }
}

#[cfg(test)]
mod test {
    use super::binomial::BinomialCoefficient;
    use ac_library::ModInt998244353 as mint;

    #[test]
    fn test_factorial() {
        let bc = BinomialCoefficient::<mint>::new(6);

        assert_eq!(bc.factorial(0), mint::new(1));
        assert_eq!(bc.factorial(1), mint::new(1));
        assert_eq!(bc.factorial(2), mint::new(2));
        assert_eq!(bc.factorial(3), mint::new(6));
        assert_eq!(bc.factorial(4), mint::new(24));
        assert_eq!(bc.factorial(5), mint::new(120));
    }

    #[test]
    fn test_binomial() {
        let bc = BinomialCoefficient::<mint>::new(6);

        assert_eq!(bc.binomial_coefficient(5, 0), mint::new(1));
        assert_eq!(bc.binomial_coefficient(5, 1), mint::new(5));
        assert_eq!(bc.binomial_coefficient(5, 2), mint::new(10));
        assert_eq!(bc.binomial_coefficient(5, 3), mint::new(10));
        assert_eq!(bc.binomial_coefficient(5, 4), mint::new(5));
        assert_eq!(bc.binomial_coefficient(5, 5), mint::new(1));
    }
}
