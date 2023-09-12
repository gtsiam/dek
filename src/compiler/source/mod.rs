mod file_loader;
pub use file_loader::FileLoader;

mod source_map;
pub use source_map::SourceMap;

use core::hash::Hash;
use miette::Diagnostic;
use thiserror::Error;
use type_map::concurrent::TypeMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct BytePos(u32);

#[derive(Debug, Error, Diagnostic)]
pub enum SourceError {
    #[error("Too much source code")]
    #[diagnostic(help(
        "If you're running into this error, it is most certainly a bug. \n\
        That, or... wow! Thanks for using dek for such a big project!. \n\
        \n\
        Still, please do let us know."
    ))]
    SourceTooLarge,

    #[error(transparent)]
    #[diagnostic(transparent)]
    Loader(#[from] Box<dyn Diagnostic + Send + Sync>),
}

#[derive(Debug)]
pub struct Source {
    /// The contents of the source.
    contents: String,

    /// The starting position for measuring spans.
    start_pos: BytePos,

    /// The context this source was loaded under.
    context: SourceContext,
}

impl Source {
    pub fn new(contents: String) -> Self {
        Self {
            contents,
            start_pos: BytePos(u32::MAX),
            context: SourceContext::new(),
        }
    }

    pub fn context(&self) -> &SourceContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut SourceContext {
        &mut self.context
    }
}

#[derive(Debug)]
pub struct SourceContext(TypeMap);

impl SourceContext {
    pub fn new() -> Self {
        Self(TypeMap::new())
    }

    pub fn extensions(&self) -> &TypeMap {
        &self.0
    }

    pub fn extensions_mut(&mut self) -> &mut TypeMap {
        &mut self.0
    }
}

/// Context used when loading the entry point.
#[derive(Debug)]
pub struct EntryContext;

pub trait SourceLoader: 'static {
    type Key: Hash + Eq + Send + Sync + 'static;
    type Error: Diagnostic + Send + Sync + 'static;

    /// Resolve the name of a source to a unique key for that source.
    fn resolve(&mut self, cx: &SourceContext, name: &str) -> Result<Self::Key, Self::Error>;

    fn load(&mut self, cx: &SourceContext, key: &Self::Key) -> Result<Source, Self::Error>;
}
