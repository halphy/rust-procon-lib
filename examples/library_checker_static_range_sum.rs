// competitive-verifier: PROBLEM https://judge.yosupo.jp/problem/static_range_sum

use proconio::input;
use rust_procon_lib::data_structure::prefix_sum::prefix_sum::PrefixSum;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let psum = PrefixSum::new(n, 0, |i| a[i]);

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        println!("{}", psum.query(l, r));
    }
}
