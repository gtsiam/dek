use core::{
    any::{Any, TypeId},
    hash::{Hash, Hasher},
};
use indexmap::{map::Entry, IndexMap};

use super::{BytePos, Source, SourceContext, SourceError, SourceLoader};

/// A source map handles storage of the loaded sources, as well as mapping between spans and
/// meaningful spans of code.
pub struct SourceMap {
    /// The list of loaded sources.
    ///
    /// Note: Preserving insertion order is vital for span lookups.
    sources: IndexMap<SourceMapKey, Source>,

    /// The method for loading sources.
    source_loader: Box<dyn SourceLoaderDyn + Send + Sync>,
}

impl SourceMap {
    pub fn new(source_loader: impl SourceLoader + Send + Sync + 'static) -> Self {
        Self {
            sources: IndexMap::new(),
            source_loader: Box::new(source_loader),
        }
    }

    pub fn load(&mut self, context: &SourceContext, name: &str) -> Result<&Source, SourceError> {
        // Compute the loader caching key for this source.
        let source_key = SourceMapKey {
            loader: self.source_loader.loader_id(),
            key: self.source_loader.resolve(context, name)?,
        };

        // Compute the start position of the source span if possible
        let start_pos = match self.sources.last() {
            Some((_, source)) => {
                let last_len = u32::try_from(source.contents.len())
                    .expect("no source already in the sourcemap should be too large");
                source.start_pos.0.checked_add(last_len)
            }
            None => Some(0),
        };

        // Return early if the source is already loaded, otherwise continue the loading process.
        match self.sources.entry(source_key) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
                let mut source = self.source_loader.load(context, &*entry.key().key)?;

                // If both start_pos and (start_pos + source_len) are valid `u32`s, we know that no span
                // computation will overflow. So we're safe to assign the source's start_pos and continue.
                if let Err(()) = match start_pos {
                    Some(start_pos) => match start_pos.checked_add(
                        u32::try_from(source.contents.len())
                            .map_err(|_| SourceError::SourceTooLarge)?,
                    ) {
                        Some(_) => {
                            source.start_pos = BytePos(start_pos);
                            Ok(())
                        }
                        None => Err(()),
                    },
                    None => Err(()),
                } {
                    return Err(SourceError::SourceTooLarge);
                }

                // At last, insert the loaded source into the source map
                Ok(entry.insert(source))
            }
        }
    }
}

struct SourceMapKey {
    loader: TypeId,
    key: Box<dyn DynKey>,
}

impl PartialEq for SourceMapKey {
    fn eq(&self, other: &Self) -> bool {
        self.loader == other.loader && *self.key == *other.key
    }
}

impl Hash for SourceMapKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.loader.hash(state);
        self.key.hash(state);
    }
}

impl Eq for SourceMapKey {}

/// Object-safe wrapper for [`SourceLoader`]
trait SourceLoaderDyn: Send + Sync {
    /// Resolve the supplied name into a DynKey.
    fn resolve(&mut self, cx: &SourceContext, name: &str) -> Result<Box<dyn DynKey>, SourceError>;

    /// Load the source corresponding to the relevant DynKey
    ///
    /// # Panics
    /// If the supplied DynKey has a different type than the one expected by this source loader.
    fn load(&mut self, cx: &SourceContext, name: &dyn DynKey) -> Result<Source, SourceError>;

    fn loader_id(&self) -> TypeId;
}

impl<T> SourceLoaderDyn for T
where
    T: SourceLoader + Send + Sync,
{
    fn resolve(&mut self, cx: &SourceContext, name: &str) -> Result<Box<dyn DynKey>, SourceError> {
        match self.resolve(cx, name) {
            Ok(key) => Ok(Box::new(key)),
            Err(err) => Err(SourceError::Loader(Box::new(err))),
        }
    }

    fn load(&mut self, cx: &SourceContext, key: &dyn DynKey) -> Result<Source, SourceError> {
        match self.load(
            cx,
            key.as_any()
                .downcast_ref::<<T as SourceLoader>::Key>()
                .unwrap(),
        ) {
            Ok(source) => Ok(source),
            Err(err) => Err(SourceError::Loader(Box::new(err))),
        }
    }

    fn loader_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

/// Hack to make arbitrary `Hash + Eq + 'static` keys
trait DynKey: Any + Send + Sync {
    fn dyn_eq(&self, other: &dyn Any) -> bool;
    fn dyn_hash(&self, state: &mut dyn Hasher);

    fn as_any(&self) -> &dyn Any;
}

impl<T> DynKey for T
where
    T: Hash + Eq + Send + Sync + 'static,
{
    fn dyn_eq(&self, other: &dyn Any) -> bool {
        match other.downcast_ref() {
            Some(other) => self == other,
            None => false,
        }
    }

    fn dyn_hash(&self, mut state: &mut dyn Hasher) {
        self.hash(&mut state);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Hash for dyn DynKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state)
    }
}

impl PartialEq for dyn DynKey {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other.as_any())
    }
}

impl Eq for dyn DynKey {}

#[cfg(test)]
mod test {
    use super::*;

    /// Ensure various types are Send + Sync. It's easy to forget when dealing with `dyn`s
    const _: () = {
        const fn is_send_sync<T: Send + Sync>() {}
        is_send_sync::<SourceMapKey>();
        is_send_sync::<SourceMap>();
        is_send_sync::<Source>();
    };
}
