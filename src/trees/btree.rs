use std::{cell::RefCell, rc::Rc};

struct BTree<T, K:Ord> {
    keys: Vec<K>,
    branches: Option<RefCell<Branches<T,K>>>,

}

struct Branches<T, K:Ord> {
    trees: Vec<Rc<BTree<T, K>>>
}