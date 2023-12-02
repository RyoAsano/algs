use super::BinTree;

#[derive(Debug)]
pub(super) struct Trunk<T, K: Ord + Copy, S> {
    pub(super) key: K,
    pub(super) value: T,
    pub(super) right: Option<BinTree<T, K, S>>,
    pub(super) left: Option<BinTree<T, K, S>>,
    pub(super) state: S,
}

impl<T, K: Ord + Copy, S> BinTree<T, K, S> {
    pub(super) fn find_parent(&self, child_key: K) -> (Option<Self>, Self, i8) {
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
}
