pub mod union_find {
    pub struct UnionFind {
        parent: Vec<usize>,
        size: Vec<usize>,
    }

    #[allow(dead_code)]
    impl UnionFind {
        pub fn new(n: usize) -> Self {
            //! Constructs a union-find tree of size `n`.

            let mut parent = vec![0; n];
            for i in 0..n {
                parent[i] = i;
            }

            Self {
                parent,
                size: vec![1; n],
            }
        }

        pub fn find(&mut self, v: usize) -> usize {
            //! Returns a root of node `v`.

            if self.parent[v] == v {
                v
            } else {
                self.parent[v] = self.find(self.parent[v]);
                self.parent[v]
            }
        }

        pub fn unite(&mut self, mut u: usize, mut v: usize) {
            //! Unites nodes `u` and `v`.

            u = self.find(u);
            v = self.find(v);

            if u == v {
                return;
            }

            if self.size[u] < self.size[v] {
                (u, v) = (v, u);
            }

            self.size[u] += self.size[v];
            self.parent[v] = u;
        }

        pub fn is_same(&mut self, u: usize, v: usize) -> bool {
            //! Returns if nodes `u` and `v` are in the same connected component.

            self.find(u) == self.find(v)
        }

        pub fn get_size(&mut self, v: usize) -> usize {
            //! Returns a size of a connected component where the node `v` belongs.
            let u = self.find(v);

            self.size[u]
        }
    }
}

#[cfg(test)]
mod test {
    use super::union_find::UnionFind;

    #[test]
    fn test_simple() {
        //! borrowed from: https://atcoder.jp/contests/practice2/tasks/practice2_a

        let mut uf = UnionFind::new(4);

        assert_eq!(uf.is_same(0, 1), false);
        uf.unite(0, 1);
        uf.unite(2, 3);
        assert_eq!(uf.is_same(0, 1), true);
        assert_eq!(uf.is_same(1, 2), false);
        uf.unite(0, 2);
        assert_eq!(uf.is_same(1, 3), true);
    }

    #[test]
    fn test_group_size() {
        let mut uf = UnionFind::new(5);

        assert_eq!(uf.get_size(2), 1);
        uf.unite(0, 2);
        uf.unite(2, 3);
        assert_eq!(uf.get_size(2), 3);
        assert_eq!(uf.get_size(1), 1);
    }
}
