use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::recorddb::RecordDb;

#[derive(Debug)]
struct Branches<T, K: Ord> {
    left: Option<Rc<BinTree<T, K>>>,
    right: Option<Rc<BinTree<T, K>>>,
}

#[derive(Debug)]
pub struct BinTree<T, K: Ord> {
    key: K,
    value: T,
    branches: RefCell<Branches<T, K>>,
}

impl<T, K: Ord + Copy> RecordDb<T, K> for BinTree<T, K> {
    fn new(key: K, value: T) -> Rc<Self> {
        Rc::new(BinTree {
            key,
            value,
            branches: RefCell::new(Branches {
                left: None,
                right: None,
            }),
        })
    }
    fn insert(self: &Rc<Self>, key: K, value: T) {
        if key < self.key {
            if self.branches.borrow().left.is_none() {
                self.branches.borrow_mut().left = Some(Self::new(key, value));
            } else {
                self.branches.borrow().left.as_ref().unwrap().insert(key, value);
            }
        } else if self.key < key {
            if self.branches.borrow().right.is_none() {
                self.branches.borrow_mut().right = Some(Self::new(key, value));
            } else {
                self.branches
                    .borrow()
                    .right
                    .as_ref()
                    .unwrap()
                    .insert(key, value);
            }
        } else {
            panic!("The key already exists.")
        }
    }

    fn delete(self: &Rc<Self>, key: K) {
        let (found, parent, pos) = self.find_parent_of(key);
        if !found {
            panic!("The given key does not exist in the tree.")
        }
        let parent = parent
            .expect("Something wrong")
            .upgrade()
            .expect("Something wrong.");
        let del_node = if pos == 0 {
            parent.branches.borrow().left.as_ref().unwrap().clone()
        } else {
            parent.branches.borrow().right.as_ref().unwrap().clone()
        };

        let repl_node: Option<Rc<BinTree<T, K>>>;
        if del_node.branches.borrow().left.is_some() & del_node.branches.borrow().right.is_some() {
            let mut bottom_tree = del_node.branches.borrow().left.as_ref().unwrap().clone();

            if bottom_tree.branches.borrow().right.is_none() {
                bottom_tree.branches.borrow_mut().right = del_node.branches.borrow_mut().right.take();
                repl_node = Some(bottom_tree);
            } else {
                let mut right = bottom_tree.branches.borrow().right.as_ref().unwrap().clone();
                loop {
                    if right.branches.borrow().right.is_some() {
                        bottom_tree = right;
                        right = bottom_tree.branches.borrow().right.as_ref().unwrap().clone();
                    } else {
                        break;
                    }
                }
                repl_node = bottom_tree.branches.borrow_mut().right.take();
                repl_node.as_ref().unwrap().branches.borrow_mut().left =
                    del_node.branches.borrow_mut().left.take();
                repl_node.as_ref().unwrap().branches.borrow_mut().right =
                    del_node.branches.borrow_mut().right.take();
            }
        } else {
            if del_node.branches.borrow().left.is_some() {
                repl_node = del_node.branches.borrow().left.clone();
            } else if del_node.branches.borrow().right.is_some() {
                repl_node = del_node.branches.borrow().right.clone();
            } else {
                repl_node = None;
            }
        }
        if pos == 0 {
            parent.branches.borrow_mut().left = repl_node;
        } else {
            parent.branches.borrow_mut().right = repl_node;
        }
    }

    fn find_parent_of(self: &Rc<Self>, cand: K) -> (bool, Option<Weak<Self>>, u32) {
        let subtree = if cand < self.key {
            self.branches.borrow().left.clone()
        } else {
            self.branches.borrow().right.clone()
        };

        if let Some(subtree) = subtree {
            if subtree.key == cand {
                return (true, Some(Rc::downgrade(self)), 0);
            } else {
                return subtree.find_parent_of(cand);
            }
        } else {
            return (false, None, 0);
        }
    }

    fn find(self: &Rc<Self>, cand: K) -> (bool, Weak<Self>) {
        if self.key == cand {
            return (true, Rc::downgrade(self));
        } else if cand < self.key {
            if self.branches.borrow().left.is_none() {
                return (false, Rc::downgrade(self));
            } else {
                return self.branches.borrow().left.as_ref().unwrap().find(cand);
            }
        } else {
            if self.branches.borrow().right.is_none() {
                return (false, Rc::downgrade(self));
            } else {
                return self.branches.borrow().right.as_ref().unwrap().find(cand);
            }
        }
    }
}