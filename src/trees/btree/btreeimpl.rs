use core::{borrow, panic};
use std::{cell::RefCell, rc::Rc};

use super::{BTree, Chi, Trunk};

impl<T, K: Ord + Copy> BTree<T, K> {
    pub fn is_full(&self) -> bool {
        self.0.len() == self.0.chicap()
    }

    pub fn new(max: usize) -> Self {
        Self(Chi::Br(Rc::new(RefCell::new(Trunk {
            keys: Vec::with_capacity(max),
            chi: Vec::with_capacity(max + 1),
        }))))
    }

    pub fn insert(self: Self, key: K, value: T) -> Self {
        if let Some(newbr) = self.insert_and_pop(key, value) {
            let mut keys = Vec::with_capacity(self.0.keycap());
            keys.extend([self.0.maxkey()]);

            let mut chi = Vec::with_capacity(self.0.chicap());
            chi.extend([self, newbr]);

            Self(Chi::Tr(Rc::new(RefCell::new(Trunk { keys, chi }))))
        } else {
            self
        }
    }

    pub fn insert_and_pop(&self, key: K, value: T) -> Option<Self> {
        let chi = self.0.as_ref();
        match chi {
            Chi::Tr(tree) => {
                // pos = i <=> k_{i-1} < key <= k_{i} <=> key \in chi_{i}
                let pos = match tree.borrow().keys.binary_search(&key) {
                    Ok(x) => x,
                    Err(x) => x,
                };
                let newchi = tree.borrow().chi[pos].insert_and_pop(key, value);
                if let Some(newchi) = newchi {
                    let new_key = tree.borrow().chi[pos].0.maxkey();
                    tree.borrow_mut().keys.insert(pos, new_key);
                    tree.borrow_mut().chi.insert(pos + 1, newchi);

                    if self.is_full() {
                        let mut keys = Vec::with_capacity(self.0.keycap());
                        let mut chi = Vec::with_capacity(self.0.chicap());
                        let at = tree.borrow().chi.len() / 2;
                        keys.extend(tree.borrow_mut().keys.split_off(at));
                        chi.extend(tree.borrow_mut().chi.split_off(at));

                        // Remove the last key because it's redundant.
                        tree.borrow_mut().keys.pop();

                        Some(Self(Chi::Tr(Rc::new(RefCell::new(Trunk { keys, chi })))))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Chi::Br(branch) => {
                let res = branch.borrow_mut().keys.binary_search(&key);
                if res.is_ok() {
                    panic!("The key already exists.");
                } else {
                    let pos = res.unwrap_err();
                    branch.borrow_mut().keys.insert(pos, key);
                    branch.borrow_mut().chi.insert(pos, value);

                    if self.is_full() {
                        let mut keys = Vec::with_capacity(self.0.keycap());
                        let mut chi = Vec::with_capacity(self.0.chicap());
                        let at = branch.borrow().keys.len() / 2;
                        keys.extend(branch.borrow_mut().keys.split_off(at));
                        chi.extend(branch.borrow_mut().chi.split_off(at));
                        Some(Self(Chi::Br(Rc::new(RefCell::new(Trunk { keys, chi })))))
                    } else {
                        None
                    }
                }
            }
        }
    }
}
