use std::{rc::Rc, cell::RefCell};

use super::Branch;

impl<T, K: Ord + Copy> Branch<T, K> {
    pub(super) fn min_size(&self) -> usize {
        self.max_size() / 2 + self.max_size() % 2
    }

    pub(super) fn max_size(&self) -> usize {
        self.vals.capacity() - 1
    }

    pub(super) fn upbd(&self) -> K {
        *self.keys.last().unwrap()
    }

    pub(super) fn insert(&mut self, key: K, value: T) -> Option<Self> {
        let pos = match self.keys.binary_search(&key) {
            Ok(_) => panic!("The key already exists."),
            Err(pos) => pos,
        };
        self.keys.insert(pos, key);
        self.vals.insert(pos, Rc::new(RefCell::new(value)));

        if self.keys.len() == self.keys.capacity() {
            let mut keys = Vec::with_capacity(self.keys.capacity());
            let mut values = Vec::with_capacity(self.vals.capacity());
            keys.extend(self.keys.split_off(self.keys.capacity() / 2));
            values.extend(self.vals.split_off(self.vals.capacity() / 2));
            Some(Self { keys, vals: values })
        } else {
            None
        }
    }

    pub(super) fn delete(&mut self, key: K) -> bool {
        let found = self.keys.binary_search(&key);
        match found {
            Ok(pos) => {
                self.keys.remove(pos);
                self.vals.remove(pos);
                true
            }
            Err(_) => false,
        }
    }
}
