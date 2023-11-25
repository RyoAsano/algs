pub trait RecordDb<T, K: Ord + Copy> {
    fn new(key: K, value: T) -> Self;

    fn insert(&self, key: K, value: T);

    fn delete(&self, key: K);

    fn find(&self, cand: K) -> (bool, K);
}
