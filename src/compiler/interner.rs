#![allow(unused)]

use std::{
    fmt,
    hash::{BuildHasher, Hash},
    marker::PhantomData,
};

use ahash::AHasher;
use indexmap::IndexSet;

pub struct Interner<T> {
    map: IndexSet<T, AHashBuilder>,
}

impl<T> Interner<T>
where
    T: Hash + Eq,
{
    pub const fn new() -> Self {
        Self {
            map: IndexSet::with_hasher(AHashBuilder),
        }
    }

    pub fn intern(&mut self, value: T) -> Interned<T> {
        let (index, _) = self.map.insert_full(value);

        Interned {
            index,
            _phantom: PhantomData,
        }
    }

    pub fn try_lookup(&self, interned: Interned<T>) -> Option<&T> {
        self.map.get_index(interned.index)
    }

    pub fn lookup(&self, interned: Interned<T>) -> &T {
        self.try_lookup(interned).expect("interned value")
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct Interned<T> {
    index: usize,
    _phantom: PhantomData<T>,
}

impl<T> fmt::Debug for Interned<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(core::any::type_name::<Self>())
    }
}

struct AHashBuilder;

impl BuildHasher for AHashBuilder {
    type Hasher = AHasher;

    fn build_hasher(&self) -> Self::Hasher {
        AHasher::default()
    }
}
