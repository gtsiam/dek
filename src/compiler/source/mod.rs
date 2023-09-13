mod file_loader;
pub mod lexer;
pub use file_loader::FileLoader;

mod source_map;
pub use source_map::SourceMap;

use core::{fmt, hash::Hash};
use miette::Diagnostic;
use thiserror::Error;
use type_map::concurrent::TypeMap;

use self::lexer::Lexer;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BytePos(u32);

#[derive(PartialEq, Eq)]
pub struct Span {
    lo: BytePos,
    hi: BytePos,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.lo.0, self.hi.0)
    }
}

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

    pub fn contents(&self) -> &str {
        &self.contents
    }

    pub fn lexer(&self) -> Lexer<'_> {
        Lexer::new(self.start_pos.0, self.contents.as_str())
    }
}

impl AsRef<str> for Source {
    fn as_ref(&self) -> &str {
        self.contents()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Spanned<T> {
    value: T,
    span: Span,
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

impl Default for SourceContext {
    fn default() -> Self {
        Self::new()
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
