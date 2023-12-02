use std::{cell::RefCell, rc::Rc};

use super::{trunk::Trunk, BinTree};

pub trait Plain<T, K> {
    fn new(key: K, value: T) -> Self;

    fn delete(&self, key: K);

    fn find(&self, cand: K) -> (bool, K);

    fn insert(&self, key: K, value: T);
}

impl<T, K: Ord + Copy> Trunk<T, K, ()> {
    pub(super) fn is_bifurcating(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }

    pub(super) fn take_rightmost_leaf(&mut self) -> BinTree<T, K, ()> {
        let right = self.right.as_ref().expect("No right branches at all");
        if right.t.borrow().right.is_some() {
            right.t.borrow_mut().take_rightmost_leaf()
        } else {
            self.right.take().unwrap()
        }
    }

    pub(super) fn take_floor_leaf(&mut self) -> BinTree<T, K, ()> {
        let left = self.left.as_ref().expect("Left branch does not exist.");
        if left.t.borrow().right.is_some() {
            left.t.borrow_mut().take_rightmost_leaf()
        } else {
            self.left.take().unwrap()
        }
    }
}

impl<T, K: Ord + Copy> Plain<T, K> for BinTree<T, K, ()> {
    fn new(key: K, value: T) -> Self {
        Self {
            t: Rc::new(RefCell::new(Trunk {
                key,
                value,
                left: None,
                right: None,
                state: (),
            })),
        }
    }

    fn insert(&self, key: K, value: T) {
        if key < self.t.borrow().key {
            if self.t.borrow().left.is_none() {
                self.t.borrow_mut().left = Some(Self::new(key, value));
            } else {
                self.t.borrow().left.as_ref().unwrap().insert(key, value);
            }
        } else if self.t.borrow().key < key {
            if self.t.borrow().right.is_none() {
                self.t.borrow_mut().right = Some(Self::new(key, value));
            } else {
                self.t
                    .borrow_mut()
                    .right
                    .as_ref()
                    .unwrap()
                    .insert(key, value);
            }
        } else {
            panic!("The key already exists.")
        }
    }

    fn delete(&self, key: K) {
        let append = |this: Option<Self>, to: &Self| {
            let this_key = this.as_ref().unwrap().t.borrow().key;
            if to.t.borrow().key == this_key {
                // In this case branch is self. Do nothing.
            } else if to.t.borrow().key < this_key {
                to.t.borrow_mut().right = this;
            } else {
                to.t.borrow_mut().left = this;
            }
        };

        let (parent, target, pos) = self.find_parent(key);
        if target.t.borrow().is_bifurcating() {
            let replacing_reaf = target.t.borrow_mut().take_floor_leaf();
            append(target.t.borrow_mut().right.take(), &replacing_reaf);
            append(target.t.borrow_mut().left.take(), &replacing_reaf);

            if let Some(parent) = parent {
                append(Some(replacing_reaf), &parent);
            }
        } else {
            if let Some(parent) = parent {
                if target.t.borrow().right.is_some() {
                    append(target.t.borrow_mut().right.take(), &parent);
                } else if target.t.borrow().left.is_some() {
                    append(target.t.borrow_mut().left.take(), &parent);
                } else {
                    // Just pull it out, no appending
                    if pos == -1 {
                        parent.t.borrow_mut().left.take();
                    } else {
                        parent.t.borrow_mut().right.take();
                    }
                }
            }
        }
    }

    fn find(&self, cand: K) -> (bool, K) {
        if self.t.borrow().key == cand {
            return (true, cand);
        } else if cand < self.t.borrow().key {
            if self.t.borrow().left.is_none() {
                return (false, self.t.borrow().key);
            } else {
                return self.t.borrow().left.as_ref().unwrap().find(cand);
            }
        } else {
            if self.t.borrow().right.is_none() {
                return (false, self.t.borrow().key);
            } else {
                return self.t.borrow().right.as_ref().unwrap().find(cand);
            }
        }
    }
}
