// competitive-verifier: PROBLEM https://judge.yosupo.jp/problem/unionfind

use proconio::input;
use rust_procon_lib::data_structure::union_find::union_find::UnionFind;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(n);

    for _ in 0..q {
        input! {
            t: usize,
            u: usize,
            v: usize,
        }

        match t {
            0 => {
                uf.unite(u, v);
            }
            1 => {
                println!("{}", if uf.is_same(u, v) { 1 } else { 0 });
            }
            _ => unreachable!(),
        }
    }
}
