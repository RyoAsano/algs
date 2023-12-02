use std::{rc::Rc, cell::RefCell, fmt::Debug};

use self::trunk::Trunk;

pub(self) mod trunk;
pub mod plain;
pub mod avl;

#[derive(Debug)]
pub struct BinTree<T, K: Ord + Copy, S> {
    pub(self) t: Rc<RefCell<Trunk<T, K, S>>>,
}