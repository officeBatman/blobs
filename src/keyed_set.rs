//! A set of elements that are given unique keys.
//!
//! A `KeyedSet<T>` has all the properties of a `HashMap<Key, T>`
//! but the keys are generated by the data structure.
//!
//! # Example
//! 
//! ```
//! use keyed_set::prelude::{KeyedSet, Key};
//! 
//! let mut set = KeyedSet::new();
//! let hi_key = set.insert("Hi!");
//! assert_eq!(set.get(hi_key), Some("Hi!"));
//! set.remove(hi_key);
//! assert_eq!(set.get(hi_key), None);
//! ```

use std::{
    collections::{
        HashMap,
        hash_map,
    },
    fmt::Display,
    marker::PhantomData,
};

/// A key is like a reference to an element of a keyed set.
///
/// The lifetime of the key is the lifetime of the creating 
/// `KeyedSet`. The time parameter is the element type of
/// the `KeyedSet`.
#[derive(Debug)]
pub struct Key<T>(usize, PhantomData<*const T>);

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
} 

impl<T> Eq for Key<T> {}

impl<T> std::hash::Hash for Key<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T> Clone for Key<T> {
    fn clone(&self) -> Self {
        Self(self.0, self.1)
    }
}

impl<T> Copy for Key<T> {}

impl<T> PartialOrd for Key<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for Key<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

/// A set of elements that are given unique keys.
///
/// A `KeyedSet<T>` has all the properties of a `HashMap<Key, T>`
/// but the keys are generated by the data structure.
///
/// # Example
/// 
/// ```
/// use keyed_set::prelude::{KeyedSet, Key};
/// 
/// let mut set = KeyedSet::new();
/// let hi_key = set.insert("Hi!");
/// assert_eq!(set.get(hi_key), Some("Hi!"));
/// set.remove(hi_key);
/// assert_eq!(set.get(hi_key), None);
/// ```
pub struct KeyedSet<T> {
    map: HashMap<Key<T>, T>,
    next: Key<T>,
}

impl<T> KeyedSet<T> {
    pub fn new() -> Self {
        Self { map: HashMap::new(), next: Key(0, PhantomData) }
    }

    fn generate_key(&mut self) -> Key<T> {
        let key = self.next;
        self.next.0 += 1;
        key
    }
    
    pub fn insert(&mut self, value: T) -> Key<T> {
        let key = self.generate_key();
        self.map.insert(key, value);
        key
    }

    pub fn get(&self, key: Key<T>) -> Option<&T> {
        self.map.get(&key)
    }
    
    pub fn get_mut(&mut self, key: Key<T>) -> Option<&mut T> {
        self.map.get_mut(&key)
    }

    pub fn remove(&mut self, key: Key<T>) -> Option<T> {
        self.map.remove(&key)
    }

    pub fn iter(&self) -> <&Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> <&mut Self as IntoIterator>::IntoIter {
        self.into_iter()
    }

    pub fn len(&self) -> usize { self.map.len() }
}

impl<T> IntoIterator for KeyedSet<T> {
    type Item = (Key<T>, T);
    type IntoIter = hash_map::IntoIter<Key<T>, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a KeyedSet<T> {
    type Item = (&'a Key<T>, &'a T);
    type IntoIter = hash_map::Iter<'a, Key<T>, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut KeyedSet<T> {
    type Item = (&'a Key<T>, &'a mut T);
    type IntoIter = hash_map::IterMut<'a, Key<T>, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter_mut()
    }
}

impl<T> Display for Key<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("#{}{}", std::any::type_name::<T>(), self.0))
    }
}

pub mod prelude {
    pub use super::{Key, KeyedSet};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_behavior() {
        let mut a = KeyedSet::new();
        let hello = a.insert("Hello!");
        let bye = a.insert("Bye!");

        assert_eq!(a.get(hello), Some(&"Hello!"));
        assert_eq!(a.get_mut(bye), Some(&mut "Bye!"));
        
        a.remove(hello);
        assert_eq!(a.get(hello), None);
        assert_eq!(a.get(bye), Some(&"Bye!"));
    }
}
