pub mod compression {
    use std::{
        collections::{HashMap, HashSet},
        hash::Hash,
    };

    pub struct Compression<T> {
        vset: HashSet<T>,
        vvec: Vec<T>,
        inv: HashMap<T, usize>,
    }

    #[allow(dead_code)]
    impl<T: Ord + Hash + Copy> Compression<T> {
        pub fn new() -> Self {
            //! Initializes the set.
            Compression {
                vset: HashSet::new(),
                vvec: vec![],
                inv: HashMap::new(),
            }
        }

        pub fn len(&self) -> usize {
            //! Returns a number of elements in the set.
            self.vset.len()
        }

        pub fn add(&mut self, x: T) {
            //! Adds `x` to the set.
            self.vset.insert(x);
        }

        pub fn add_from_vec(&mut self, vec: &Vec<T>) {
            //! Adds all elements of `vec` to the set.
            for &x in vec {
                self.add(x);
            }
        }

        pub fn compress(&mut self) {
            //! Conducts a coordinate compression.

            for v in &self.vset {
                self.vvec.push(*v);
            }

            self.vvec.sort();

            for (i, v) in self.vvec.iter().enumerate() {
                self.inv.insert(*v, i);
            }
        }

        pub fn get_index(&self, x: T) -> Option<&usize> {
            //! Returns the index of `x` (indexed in ascending order).
            self.inv.get(&x)
        }

        pub fn get_value(&self, idx: usize) -> T {
            //! Returns the element of index `idx` (indexed in ascending order).
            self.vvec[idx]
        }
    }
}

#[cfg(test)]
mod test {
    use super::compression::Compression;

    #[test]
    fn test_simple() {
        let a = vec![3, 3, 1, 6, 1];

        let mut cpr = Compression::new();
        cpr.add_from_vec(&a);
        cpr.compress();

        assert_eq!(cpr.get_index(3), Some(&1));
        assert_eq!(cpr.get_index(2), None);
        assert_eq!(cpr.get_value(0), 1);
        assert_eq!(cpr.get_value(2), 6);
    }
}
