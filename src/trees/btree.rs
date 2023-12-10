mod branch;
mod trunk;

use std::{
    cell::RefCell,
    rc::Rc,
};

#[derive(Debug)]
pub struct Trunk<T, K: Ord> {
    keys: Vec<K>,
    vals: Vec<BTree<T, K>>,
    upbd: K,
}

#[derive(Debug)]
pub struct Branch<T, K: Ord> {
    keys: Vec<K>,
    vals: Vec<Rc<RefCell<T>>>,
}

#[derive(Debug)]
pub enum BTree<T, K: Ord> {
    Tr(Rc<RefCell<Trunk<T, K>>>),
    Br(Rc<RefCell<Branch<T, K>>>),
}

/*
You might think that you could replace Trunk<T, K> by  Branch<BTree<T, K>, K>. But in this case the methods would be duplicated:

impl <T, K> Branch<T, K> {
    fn insert(&self, key: K, value: T) {
        // Impl. for Branch
    }
}

impl <T, K> Branch<BTree<T, K>, K> {
    fn insert(&self, key: K, value: T) {
        // Impl. for Trunk, which is a duplicate of the above insert method.
        // But the algorithm in this method is essentially different from it.
    }
}

 */

impl<T, K: Ord + Copy> BTree<T, K> {
    fn unwrap_tr(&self) -> &Rc<RefCell<Trunk<T, K>>> {
        match self {
            BTree::Tr(tr) => tr,
            _ => panic!("Cannot unwrap it as a trunk."),
        }
    }

    fn unwrap_br(&self) -> &Rc<RefCell<Branch<T, K>>> {
        match self {
            BTree::Br(br) => br,
            _ => panic!("Cannot unwrap it as a trunk."),
        }
    }

    pub fn new(max: usize) -> Self {
        Self::Br(Rc::new(RefCell::new(Branch {
            keys: Vec::with_capacity(max + 1),
            vals: Vec::with_capacity(max + 1),
        })))
    }

    pub fn find(&self, key: K) -> Result<Rc<RefCell<T>>, String> {
        match self {
            BTree::Br(ref br) => match br.borrow().keys.binary_search(&key) {
                Ok(pos) => Ok(br.borrow().vals[pos].clone()),
                Err(_) => Err("Not found".to_string()),
            },
            BTree::Tr(ref tr) => {
                let pos = match tr.borrow().keys.binary_search(&key) {
                    Ok(pos) => pos,
                    Err(pos) => pos,
                };
                tr.borrow().vals[pos].find(key)
            }
        }
    }

    pub fn insert(&mut self, key: K, value: T) {
        match self {
            Self::Br(br) => {
                let right_br = br.borrow_mut().insert(key, value);
                if let Some(right_br) = right_br {
                    let mut keys = Vec::with_capacity(right_br.keys.capacity());
                    let mut vals = Vec::with_capacity(right_br.vals.capacity());
                    let upbd = right_br.upbd();
                    keys.push(*br.borrow().keys.iter().max().unwrap());
                    vals.extend([
                        Self::Br(br.clone()),
                        Self::Br(Rc::new(RefCell::new(right_br))),
                    ]);
                    *self = Self::Tr(Rc::new(RefCell::new(Trunk { keys, vals, upbd })));
                }
            }
            Self::Tr(tr) => {
                let right_tr = tr.borrow_mut().insert(key, value);
                if let Some(right_tr) = right_tr {
                    let mut keys = Vec::with_capacity(right_tr.keys.capacity());
                    let mut vals = Vec::with_capacity(right_tr.vals.capacity());
                    let upbd = right_tr.upbd;
                    keys.push(tr.borrow().upbd);
                    vals.extend([
                        Self::Tr(tr.clone()),
                        Self::Tr(Rc::new(RefCell::new(right_tr))),
                    ]);

                    *self = Self::Tr(Rc::new(RefCell::new(Trunk { keys, vals, upbd })));
                } else {
                    if tr.borrow().upbd < key {
                        tr.borrow_mut().upbd = key;
                    }
                }
            }
        }
    }

    pub fn delete(&mut self, key: K) -> bool {
        match self {
            BTree::Br(br) => {
                return br.borrow_mut().delete(key);
            }
            BTree::Tr(tr) => {
                let found = tr.borrow_mut().delete(key);
                if !found {
                    return false;
                }
                if tr.borrow().keys.len() == 0 {
                    let it = tr.borrow_mut().vals.pop().unwrap();
                    *self = it;
                }
                return true;
            }
        }
    }
}
