use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::hash::Hash;
use crate::common::contract::{Get, GetMut, Insert, InsertSimple};

#[derive(Clone, Debug)]
pub struct Pool<K: PoolKey, V> {
    items: HashMap<K, V>,
    next_id: K,
}

pub trait PoolKey: Hash + PartialEq + Eq + Copy {
    fn initial() -> Self;
    fn next(&self) -> Self;
}

impl<K: PoolKey, V> Insert<K, V> for Pool<K, V> {
    fn insert(&mut self, value: V) -> K {
        let key = self.next_id;
        self.next_id = key.next();
        self.items.insert(key, value);
        key
    }
}

impl<K: PoolKey, V> InsertSimple<V> for Pool<K, V> {
    fn insert_simple(&mut self, value: V) {
        self.insert(value);
    }
}

impl<K: PoolKey, V> Pool<K, V> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            next_id: K::initial(),
        }
    }

    pub fn remove(&mut self, id: K) -> Option<V> {
        self.items.remove(&id)
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        self.items.iter_mut()
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl <K: PoolKey, V> Get<K,V> for Pool<K,V> {
    #[inline]
    fn get(&self, k: &K) -> Option<&V> {
        self.items.get(k)
    }
}

impl <K: PoolKey, V> GetMut<K,V> for Pool<K,V> {
    #[inline]
    fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.items.get_mut(k)
    }
}

impl <K: PoolKey, V, T: Into<Vec<V>>> From<T> for Pool<K, V> {
    fn from(items: T) -> Self {
        let mut pool = Pool::new();
        for item in items.into() {
            pool.insert(item);
        }
        pool
    }
}
