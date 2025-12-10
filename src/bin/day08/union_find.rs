use std::{collections::HashMap, hash::Hash};

pub type Id = usize;
pub struct DisjointSet<T: Eq + Hash> {
    locs: HashMap<T, Id>,
    connections: Vec<Id>,
    sizes: HashMap<Id, usize>,
}

impl<T: Eq + Hash> DisjointSet<T> {
    pub fn new() -> Self {
        Self {
            locs: HashMap::new(),
            connections: vec![],
            sizes: HashMap::new(),
        }
    }
    pub fn singleton(&mut self, t: T) -> Option<Id> {
        if self.locs.contains_key(&t) {
            None
        } else {
            assert!(self.locs.len() == self.connections.len());
            let i = self.connections.len();
            self.locs.insert(t, i);
            self.connections.push(i);
            let ok = self.sizes.insert(i, 1).is_none();
            assert!(ok);
            Some(i)
        }
    }
    fn compress(&mut self, first: Id, root: Id) {
        let mut i = first;
        while i != root {
            let next = self.connections[i];
            self.connections[i] = root;
            i = next;
        }
        assert!(self.connections[i] == i);
    }
    pub fn find(&mut self, t: &T) -> Option<Id> {
        if let Some(mut i) = self.locs.get(t).copied() {
            let first = i;
            while self.connections[i] != i {
                i = self.connections[i];
            }
            self.compress(first, i);
            Some(i)
        } else {
            None
        }
    }
    pub fn union(&mut self, t1: &T, t2: &T) {
        if let Some(i) = self.find(t1)
            && let Some(i2) = self.find(t2)
            && i != i2
        {
            self.connections[i2] = i;
            *self.sizes.get_mut(&i).unwrap() += self.sizes[&i2];
            self.sizes.remove(&i2);
        }
    }
    pub fn size(&mut self, i: Id) -> Option<usize> {
        self.sizes.get(&i).cloned()
    }
}
