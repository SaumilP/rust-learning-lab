//! Proxy: control access to another object through the same interface.

use std::cell::RefCell;
use std::collections::HashMap;

trait UserStore {
    fn find_name(&self, id: u32) -> Option<String>;
}

struct Database;

impl UserStore for Database {
    fn find_name(&self, id: u32) -> Option<String> {
        (id == 1).then(|| "Ferris".to_string())
    }
}

struct CachedStore<S> {
    inner: S,
    cache: RefCell<HashMap<u32, String>>,
}

impl<S: UserStore> UserStore for CachedStore<S> {
    fn find_name(&self, id: u32) -> Option<String> {
        if let Some(name) = self.cache.borrow().get(&id) {
            return Some(name.clone());
        }
        let name = self.inner.find_name(id)?;
        self.cache.borrow_mut().insert(id, name.clone());
        Some(name)
    }
}

fn main() {
    let store = CachedStore {
        inner: Database,
        cache: RefCell::new(HashMap::new()),
    };
    println!("First lookup: {:?}", store.find_name(1));
    println!("Cached lookup: {:?}", store.find_name(1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proxy_caches_the_real_subject_result() {
        let store = CachedStore {
            inner: Database,
            cache: RefCell::new(HashMap::new()),
        };
        assert_eq!(store.find_name(1).as_deref(), Some("Ferris"));
        assert_eq!(
            store.cache.borrow().get(&1).map(String::as_str),
            Some("Ferris")
        );
    }
}
