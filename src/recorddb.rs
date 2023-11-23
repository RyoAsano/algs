use std::rc::{Rc, Weak};

pub trait RecordDb<T, K: Ord + Copy>{
    fn new(key: K, value: T) -> Rc<Self>;

    fn insert(self: &Rc<Self>, key: K, value: T);

    fn delete(self: &Rc<Self>, key: K);

    fn find_parent_of(self: &Rc<Self>, cand: K) -> (bool, Option<Weak<Self>>, u32);

    fn find(self: &Rc<Self>, cand: K) -> (bool, Weak<Self>);
}