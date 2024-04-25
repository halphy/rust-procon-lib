// competitive-verifier: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/2/DSL_2_A

use proconio::input;
use rust_procon_lib::data_structure::segment_tree::segment_tree::SegmentTree;

const INF: usize = (1 << 31) - 1;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut seg = SegmentTree::new(n, INF, |x, y| x.min(y));
    seg.construct(&vec![INF; n]);

    for _ in 0..q {
        input! {
            t: usize,
        }

        match t {
            0 => {
                input! {
                    i: usize,
                    x: usize,
                }

                seg.update(i, x);
            }
            1 => {
                input! {
                    l: usize,
                    r: usize,
                }

                println!("{}", seg.prod(l, r + 1));
            }
            _ => unreachable!(),
        }
    }
}
