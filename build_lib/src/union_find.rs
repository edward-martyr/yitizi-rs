use std::{cmp::Eq, collections::HashMap, hash::Hash};

pub trait HashKeyClone: Clone + Eq + Hash {}
impl<T: Clone + Eq + Hash> HashKeyClone for T {}

#[derive(Debug)]
pub struct UnionFind<T: HashKeyClone> {
    elems: Vec<T>,
    index: HashMap<T, usize>,
    uf: Vec<isize>,
}

impl<T: HashKeyClone> UnionFind<T> {
    pub fn new(it: Option<impl Iterator<Item = T>>) -> Self {
        let mut _self = Self::default();
        if let Some(it) = it {
            for elem in it {
                _self.add(elem);
            }
        }
        _self
    }

    pub fn add(&mut self, elem: T) -> bool {
        if self.index.contains_key(&elem) {
            return false;
        }
        let idx = self.uf.len();
        self.index.insert(elem.clone(), idx);
        self.elems.push(elem.clone());
        self.uf.push(-1);
        true
    }

    pub fn find(&mut self, elem: T) -> T {
        let root = self.find_idx(self.index[&elem]);
        self.elems[root].clone()
    }

    pub fn union(&mut self, elem1: T, elem2: T) -> bool {
        let idx1 = self.index.get(&elem1).copied().expect("elem1 not found");
        let idx2 = self.index.get(&elem2).copied().expect("elem2 not found");
        self.union_idx(idx1, idx2)
    }

    pub fn same_set(&mut self, elem1: T, elem2: T) -> bool {
        let idx1 = self.index.get(&elem1).copied().expect("elem1 not found");
        let idx2 = self.index.get(&elem2).copied().expect("elem2 not found");
        self.find_idx(idx1) == self.find_idx(idx2)
    }

    pub fn dump(&mut self) -> HashMap<T, Vec<T>> {
        let mut sets = HashMap::new();
        let uf = &self.uf.clone();

        uf.iter().enumerate().for_each(|(idx, _)| {
            let root = self.find_idx(idx);
            let elem = &self.elems[idx];
            let set = sets
                .entry(self.elems[root].clone())
                .or_insert_with(Vec::new);
            set.push(elem.clone());
        });

        sets
    }

    fn find_idx(&mut self, idx: usize) -> usize {
        if self.uf[idx] < 0 {
            idx
        } else {
            let root = self.find_idx(self.uf[idx] as usize);
            self.uf[idx] = root as isize;
            root
        }
    }

    fn union_idx(&mut self, idx1: usize, idx2: usize) -> bool {
        let root1 = self.find_idx(idx1);
        let root2 = self.find_idx(idx2);
        if root1 == root2 {
            return false;
        }
        if self.uf[root1] < self.uf[root2] {
            self.uf[root1] += self.uf[root2];
            self.uf[root2] = root1 as isize;
        } else {
            self.uf[root2] += self.uf[root1];
            self.uf[root1] = root2 as isize;
        }
        true
    }
}

impl<T: HashKeyClone> Default for UnionFind<T> {
    fn default() -> Self {
        Self {
            elems: Vec::new(),
            index: HashMap::new(),
            uf: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::default();
        uf.add(1);
        uf.add(2);
        uf.add(3);
        uf.union(1, 2);
        assert_eq!(uf.same_set(1, 2), true);
        assert_eq!(uf.same_set(1, 3), false);
    }
}
