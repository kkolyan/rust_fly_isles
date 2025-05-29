use std::collections::HashMap;
use std::hash::Hash;

use enum_iterator::Sequence;
use futures::future::LocalBoxFuture;
use futures::FutureExt;

pub fn new_enum_map_async<'a, K, V, F>(f: F) -> LocalBoxFuture<'a, HashMap<K, V>>
    where F: Fn(K) -> LocalBoxFuture<'a, V> + 'a,
          K: Sequence + Eq + Hash + Clone
{
    async move {
        let mut result = HashMap::with_capacity(enum_iterator::cardinality::<K>());
        for k in enum_iterator::all::<K>() {
            let v = f(k.clone()).await;
            result.insert(k, v);
        }
        result
    }.boxed_local()
}

pub fn new_enum_map<K, V, F>(f: F) -> HashMap<K, V>
    where F: Fn(K) -> V,
          K: Sequence + Eq + Hash + Clone
{
    let mut result = HashMap::with_capacity(enum_iterator::cardinality::<K>());
    for k in enum_iterator::all::<K>() {
        let v = f(k.clone());
        result.insert(k, v);
    }
    result
}
