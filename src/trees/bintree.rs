use std::{cell::RefCell, rc::Rc};

use crate::recorddb::RecordDb;

#[derive(Debug)]
struct BinTreeImpl<T, K: Ord + Copy> {
    key: K,
    value: T,
    right: Option<BinTree<T, K>>,
    left: Option<BinTree<T, K>>,
}

#[derive(Debug)]
pub struct BinTree<T, K: Ord + Copy> {
    t: Rc<RefCell<BinTreeImpl<T, K>>>,
}

impl<T, K: Ord + Copy> BinTreeImpl<T, K> {
    fn is_bifurcating(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }

    fn take_right(&mut self) -> BinTree<T, K> {
        self.right.take().unwrap()
    }

    fn take_left(&mut self) -> BinTree<T, K> {
        self.left.take().unwrap()
    }

    fn take_rightmost_leaf(&mut self) -> BinTree<T, K> {
        let right = self.right.as_ref().expect("No right branches at all");
        if right.t.borrow().right.is_some() {
            right.t.borrow_mut().take_rightmost_leaf()
        } else {
            self.take_right()
        }
    }

    fn take_floor_leaf(&mut self) -> BinTree<T, K> {
        let left = self.left.as_ref().expect("Left branch does not exist.");
        if left.t.borrow().right.is_some() {
            left.t.borrow_mut().take_rightmost_leaf()
        } else {
            self.left.take().unwrap()
        }
    }
}

impl<T, K: Ord + Copy> BinTree<T, K> {
    fn find_parent(&self, child_key: K) -> (Option<Self>, Self, i8) {
        if self.t.borrow().key < child_key {
            if let Some(ref right) = self.t.borrow().right {
                if right.t.borrow().key == child_key {
                    (
                        Some(Self { t: self.t.clone() }),
                        Self { t: right.t.clone() },
                        1,
                    )
                } else {
                    Self::find_parent(right, child_key)
                }
            } else {
                panic!("Failed to find the key.")
            }
        } else if child_key < self.t.borrow().key {
            if let Some(ref left) = self.t.borrow().left {
                if left.t.borrow().key == child_key {
                    (
                        Some(Self { t: self.t.clone() }),
                        Self { t: left.t.clone() },
                        -1,
                    )
                } else {
                    Self::find_parent(left, child_key)
                }
            } else {
                panic!("Failed to find the key.")
            }
        } else {
            (None, Self { t: self.t.clone() }, 0)
        }
    }

    fn append(&self, branch: Self) {
        if self.t.borrow().key == branch.t.borrow().key {
            // In this case branch is self. Do nothing.
        } else if self.t.borrow().key < branch.t.borrow().key {
            self.t.borrow_mut().right = Some(branch);
        } else {
            self.t.borrow_mut().left = Some(branch);
        }
    }
}

impl<T, K: Ord + Copy> RecordDb<T, K> for BinTree<T, K> {
    fn new(key: K, value: T) -> Self {
        Self {
            t: Rc::new(RefCell::new(BinTreeImpl {
                key,
                value,
                left: None,
                right: None,
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
        let (parent, target, pos) = self.find_parent(key);
        if target.t.borrow().is_bifurcating() {
            let replacing_reaf = target.t.borrow_mut().take_floor_leaf();
            replacing_reaf.append(target.t.borrow_mut().take_right());
            replacing_reaf.append(target.t.borrow_mut().take_left());

            if let Some(parent) = parent {
                parent.append(replacing_reaf);
            }
        } else {
            if let Some(parent) = parent {
                if target.t.borrow().right.is_some() {
                    parent.append(target.t.borrow_mut().take_right());
                } else if target.t.borrow().left.is_some() {
                    parent.append(target.t.borrow_mut().take_left());
                } else {
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
