use std::{cell::RefCell, rc::Rc};

use super::{BTree, Trunk};

impl<T, K: Ord + Copy> Trunk<T, K> {
    fn min_size(&self) -> usize {
        self.max_size() / 2 + self.max_size() % 2
    }

    fn max_size(&self) -> usize {
        self.vals.capacity() - 1
    }

    pub(super) fn insert(&mut self, mut key: K, value: T) -> Option<Self> {
        let pos = match self.keys.binary_search(&key) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };
        match self.vals[pos] {
            BTree::Br(ref br) => {
                let branch = br.borrow_mut().insert(key, value);
                if let Some(branch) = branch {
                    self.keys
                        .insert(pos, *br.borrow().keys.iter().max().unwrap());
                    self.vals
                        .insert(pos + 1, BTree::Br(Rc::new(RefCell::new(branch))));
                    self.upbd = self.vals.last().unwrap().unwrap_br().borrow().upbd();
                };
            }
            BTree::Tr(ref tr) => {
                let trunk = tr.borrow_mut().insert(key, value);
                if let Some(trunk) = trunk {
                    // Replace the existing key at pos.
                    self.keys.insert(pos, trunk.upbd);
                    self.vals
                        .insert(pos + 1, BTree::Tr(Rc::new(RefCell::new(trunk))));
                    self.upbd = self.vals.last().unwrap().unwrap_tr().borrow().upbd;
                };
            }
        };
        if self.keys.len() == self.keys.capacity() {
            let mut keys = Vec::with_capacity(self.keys.capacity());
            let mut vals = Vec::with_capacity(self.vals.capacity());
            let upbd = self.upbd;
            keys.extend(self.keys.split_off(self.vals.len() / 2));
            vals.extend(self.vals.split_off(self.vals.len() / 2));
            // Remove the last key if it is tree as it's redundant.
            self.upbd = self.keys.pop().unwrap();

            Some(Self { keys, vals, upbd })
        } else {
            None
        }
    }

    pub(super) fn delete(&mut self, key: K) -> bool {
        let mut pos = match self.keys.binary_search(&key) {
            Ok(pos) => pos,
            Err(pos) => pos,
        };

        match self.vals[pos] {
            BTree::Br(ref br) => {
                if !br.borrow_mut().delete(key) {
                    return false;
                } else if br.borrow().min_size() <= br.borrow().vals.len() {
                    return true;
                }
            }
            BTree::Tr(ref tr) => {
                if !tr.borrow_mut().delete(key) {
                    return false;
                } else if tr.borrow().min_size() <= tr.borrow().vals.len() {
                    return true;
                };
            }
        };

        // Here we merge the tree with another.
        // We base our target index on the lower one.
        if pos == self.vals.len() - 1 {
            pos = pos - 1;
        };
        let lower = self.vals.remove(pos);
        let upper = self.vals.remove(pos);

        match (lower, upper) {
            (BTree::Br(lower), BTree::Br(upper)) => {
                // remove the key at pos
                self.keys.remove(pos);

                let mut lks = Vec::with_capacity(2 * lower.borrow().keys.capacity());
                lks.extend(lower.borrow_mut().keys.drain(..));
                lks.extend(upper.borrow_mut().keys.drain(..));

                let mut lvs = Vec::with_capacity(2 * lower.borrow().vals.capacity());
                lvs.extend(lower.borrow_mut().vals.drain(..));
                lvs.extend(upper.borrow_mut().vals.drain(..));

                let upper = if self.max_size() < lvs.len() {
                    let mut uks = Vec::with_capacity(upper.borrow().keys.capacity());
                    let mut uvs = Vec::with_capacity(upper.borrow().vals.capacity());
                    uks.extend(lks.split_off(lks.len() / 2));
                    uvs.extend(lvs.split_off(lvs.len() / 2));
                    let lower = super::Branch {
                        keys: lks,
                        vals: lvs,
                    };
                    let upper = super::Branch {
                        keys: uks,
                        vals: uvs,
                    };
                    self.keys.insert(pos, lower.upbd());
                    // Inserts upper first.
                    self.vals
                        .insert(pos, BTree::Br(Rc::new(RefCell::new(upper))));
                    // ... and then lower.
                    self.vals
                        .insert(pos, BTree::Br(Rc::new(RefCell::new(lower))));
                } else {
                    let lower = super::Branch {
                        keys: lks,
                        vals: lvs,
                    };
                    self.vals
                        .insert(pos, BTree::Br(Rc::new(RefCell::new(lower))));
                };
            }
            (BTree::Tr(lower), BTree::Tr(upper)) => {
                let mut lks = Vec::with_capacity(2 * self.max_size());
                let mut lvs = Vec::with_capacity(2 * self.max_size());

                lks.extend(lower.borrow_mut().keys.drain(..));
                lks.extend(upper.borrow_mut().keys.drain(..));

                lvs.extend(lower.borrow_mut().vals.drain(..));
                lvs.extend(upper.borrow_mut().vals.drain(..));

                if self.max_size() < lvs.len() {
                    let mut uks = Vec::with_capacity(upper.borrow().keys.capacity());
                    let mut uvs = Vec::with_capacity(upper.borrow().vals.capacity());
                    uks.extend(lks.split_off(lks.len() / 2));
                    uvs.extend(lvs.split_off(lvs.len() / 2));

                    lks.shrink_to(lower.borrow().keys.capacity());
                    lvs.shrink_to(lower.borrow().vals.capacity());

                    let lupbd = lvs.last().unwrap().unwrap_tr().borrow().upbd;
                    let lower = Self {
                        keys: lks,
                        vals: lvs,
                        upbd: lupbd,
                    };
                    let uupbd = uvs.last().unwrap().unwrap_tr().borrow().upbd;
                    let upper = Self {
                        keys: uks,
                        vals: uvs,
                        upbd: uupbd,
                    };
                    self.keys.insert(pos, lower.upbd);
                    // Inserts upper first.
                    self.vals
                        .insert(pos, BTree::Tr(Rc::new(RefCell::new(upper))));
                    // ...and then lower.
                    self.vals
                        .insert(pos, BTree::Tr(Rc::new(RefCell::new(lower))));
                } else {
                    lks.shrink_to(lower.borrow().keys.capacity());
                    lvs.shrink_to(lower.borrow().vals.capacity());

                    let lupbd = lvs.last().unwrap().unwrap_tr().borrow().upbd;
                    let lower = Self {
                        keys: lks,
                        vals: lvs,
                        upbd: lupbd,
                    };
                    self.vals
                        .insert(pos, BTree::Tr(Rc::new(RefCell::new(lower))));
                };
            }
            _ => panic!("Something wrong with the algorithm."),
        };

        return true;
    }
}
