use std::collections::VecDeque;

pub trait Get<K, V> {
    fn get(&self, value: &K) -> Option<&V>;
}

pub trait GetMut<K, V> {
    fn get_mut(&mut self, value: &K) -> Option<&mut V>;
}

pub trait Insert<K, V> {
    fn insert(&mut self, value: V) -> K;
}

pub trait InsertSimple<V> {
    fn insert_simple(&mut self, value: V);
}

impl<V> InsertSimple<V> for Vec<V> {
    #[inline]
    fn insert_simple(&mut self, value: V) {
        self.push(value);
    }
}

impl<V> InsertSimple<V> for VecDeque<V> {
    #[inline]
    fn insert_simple(&mut self, value: V) {
        self.push_back(value);
    }
}
