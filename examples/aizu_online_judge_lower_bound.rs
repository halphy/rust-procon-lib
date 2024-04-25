// competitive-verifier: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/8/ITP2/6/ITP2_6_C

use proconio::input;
use rust_procon_lib::utils::binary_search::binary_search::lower_bound;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    }

    for _ in 0..q {
        input! {
            k: usize,
        }

        println!("{}", lower_bound(&a, k));
    }
}
