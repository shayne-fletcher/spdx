use anyhow::Result;

use std::hash::Hash;

pub struct NonEvictingStore<K: Hash + Eq, V> {
    store: dashmap::DashMap<K, V>,
}

impl<K: Hash + Eq, V> Default for NonEvictingStore<K, V> {
    fn default() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

impl<K: Hash + Eq, V> NonEvictingStore<K, V> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<K, V> super::Store<K, V> for NonEvictingStore<K, V>
where
    K: Send + Sync + Hash + Eq,
    V: Clone + Send + Sync,
{
    fn contains_key(&self, key: &K) -> Result<bool> {
        Ok(self.store.contains_key(key))
    }

    fn get(&self, key: &K) -> Result<Option<V>> {
        Ok(self.store.get(key).map(|x| V::clone(&*x)))
    }

    fn insert(&self, key: K, val: V) -> Result<()> {
        self.store.insert(key, val);
        Ok(())
    }
}

impl<K: Hash + Eq, V> std::fmt::Debug for NonEvictingStore<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NonEvictingStore").finish()
    }
}
