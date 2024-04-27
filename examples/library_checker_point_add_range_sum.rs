// competitive-verifier: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::input;
use rust_procon_lib::data_structure::segment_tree::segment_tree::SegmentTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut seg = SegmentTree::new(n, 0, |x, y| x + y);
    seg.construct(&a);

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            0 => {
                input! {
                    p: usize,
                    x: usize,
                }

                seg.update(p, seg.get(p) + x);
            }
            1 => {
                input! {
                    l: usize,
                    r: usize,
                }

                println!("{}", seg.prod(l, r));
            }
            _ => unreachable!(),
        }
    }
}
