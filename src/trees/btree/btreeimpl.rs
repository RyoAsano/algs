use core::{borrow, panic};
use std::{cell::RefCell, mem::swap, rc::Rc};

use super::{BTree, Chi, Trunk};

#[repr(u8)]
enum DelOp {
    None,
    NeedMerge,
}

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

    fn merge_chi(&self, mut low: usize, mut high: usize) -> DelOp {
        if low == high {
            panic!("illigal use.");
        } else if low > high {
            swap(&mut low, &mut high);
        }

        let tree = self.0.unwrap_tr();
        let mut newkeys = Vec::with_capacity(self.0.keycap() * 2);
        newkeys.extend(
            tree.borrow().chi[low]
                .0
                .unwrap_tr()
                .borrow_mut()
                .keys
                .drain(..),
        );
        newkeys.extend(
            tree.borrow().chi[high]
                .0
                .unwrap_tr()
                .borrow_mut()
                .keys
                .drain(..),
        );

        let mut newchi = Vec::with_capacity(self.0.chicap() * 2);
        newchi.extend(
            tree.borrow().chi[low]
                .0
                .unwrap_tr()
                .borrow_mut()
                .chi
                .drain(..),
        );
        newchi.extend(
            tree.borrow().chi[high]
                .0
                .unwrap_tr()
                .borrow_mut()
                .chi
                .drain(..),
        );
        if self.0.chicap() < newchi.len() {
            let at = newchi.len() / 2;
            let mut newhighkeys = Vec::with_capacity(self.0.chicap());
            let mut newhighchi = Vec::with_capacity(self.0.chicap());
            newhighkeys.extend(newkeys.split_off(at));
            newhighchi.extend(newchi.split_off(at));
            tree.borrow_mut().chi[high] = Self(Chi::Tr(Rc::new(RefCell::new(Trunk {
                keys: newhighkeys,
                chi: newhighchi,
            }))));
        } else {
            tree.borrow_mut().chi.remove(high);
        }
        newkeys.shrink_to(self.0.keycap());
        newchi.shrink_to(self.0.chicap());
        newkeys.pop();
        tree.borrow_mut().chi.insert(
            low,
            Self(Chi::Tr(Rc::new(RefCell::new(Trunk {
                keys: newkeys,
                chi: newchi,
            })))),
        );
        todo!()
    }

    fn delete(&self, key: K) -> DelOp {
        let chi = self.0.as_ref();
        match chi {
            Chi::Tr(tree) => {
                let pos = match tree.borrow().keys.binary_search(&key) {
                    Ok(pos) => pos,
                    Err(pos) => pos,
                };
                let res = tree.borrow().chi[pos].delete(key);
                if let DelOp::NeedMerge = res {
                    if pos == 0 {
                        self.merge_chi(0, 1);
                    } else {
                        self.merge_chi(pos - 1, pos);
                    };
                    let min = self.0.chicap();
                    let min = min / 2 + min % 2;
                    if tree.borrow().chi.len() < min {
                        DelOp::NeedMerge
                    } else {
                        DelOp::None
                    }
                } else {
                    DelOp::None
                }
            }
            Chi::Br(branch) => {
                let res = branch.borrow().keys.binary_search(&key);
                if res.is_err() {
                    panic!("The key was not found.");
                }
                let pos = res.unwrap();
                branch.borrow_mut().keys.remove(pos);
                branch.borrow_mut().chi.remove(pos);

                let min = branch.borrow().chi.capacity();
                let min = min / 2 + min % 2;
                if branch.borrow().chi.len() < min {
                    DelOp::NeedMerge
                } else {
                    DelOp::None
                }
            }
        }
    }
}
