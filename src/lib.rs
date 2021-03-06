pub mod element;
pub mod semigroup;
pub(crate) mod utils;

use std::collections::hash_map::{DefaultHasher, HashMap};
use std::collections::HashSet;
use std::hash::BuildHasherDefault;

/// A type of HashMap that uses a determined seed
pub type DetHashMap<K, V> = HashMap<K, V, BuildHasherDefault<DefaultHasher>>;

/// A type of DetHashSet that uses a determined seed
pub type DetHashSet<K> = HashSet<K, BuildHasherDefault<DefaultHasher>>;
