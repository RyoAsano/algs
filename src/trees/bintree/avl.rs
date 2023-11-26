use crate::recorddb::RecordDb;

use super::treeimpl::{BinTreeImpl, Bal};

#[derive(Debug)]
pub struct AvlTree<T, K: Ord + Copy>(BinTreeImpl<T, K>);

impl<T, K: Ord + Copy> RecordDb<T, K> for AvlTree<T, K> {
    fn new(key: K, value: T) -> Self {
        Self(BinTreeImpl::new(key, value))
    }

    fn delete(&self, key: K) {
        self.0.delete(key, Bal::AVL);
    }

    fn find(&self, cand: K) -> (bool, K) {
        self.0.find(cand)
    }

    fn insert(&self, key: K, value: T) {
        self.0.insert(key, value, Bal::AVL);
    }
}