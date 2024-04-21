pub mod binary_search {
    pub fn binary_search<F: Fn(usize) -> bool>(initial_t: usize, initial_f: usize, f: F) -> usize {
        //! Binary-searches a minimum (or maximum) `x` such that `f(x) == true` between `initial_t` and `initial_f`.

        let mut ok = initial_t;
        let mut ng = initial_f;

        while ok.abs_diff(ng) > 1 {
            let mid = (ok + ng) / 2;

            if f(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }

    pub fn lower_bound<T: PartialOrd>(a: &Vec<T>, x: T) -> usize {
        //! Returns the minimum index `i` such that `a[i] >= x` or `a.len()` if `i` does not exist.

        binary_search(a.len() + 1, 0, |i| a[i - 1] >= x) - 1
    }
}
