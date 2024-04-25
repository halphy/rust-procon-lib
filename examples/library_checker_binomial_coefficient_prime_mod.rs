// competitive-verifier: PROBLEM https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod

use ac_library::ModInt as mint;
use proconio::input;
use rust_procon_lib::math::binomial::binomial::BinomialCoefficient;

const SIZE: usize = 10_000_000;

fn main() {
    input! {
        t: usize,
        m: u32,
    }

    mint::set_modulus(m);

    let bc = BinomialCoefficient::<mint>::new(SIZE);

    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
        }

        println!("{}", bc.binomial_coefficient(n, k));
    }
}
