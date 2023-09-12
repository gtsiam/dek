use miette::Diagnostic;
use thiserror::Error;

use self::source::{EntryContext, FileLoader, SourceContext, SourceError, SourceMap};

pub mod parse;
pub mod source;

pub struct Compiler {
    /// The entry point of the program.
    entry: String,

    /// The source map.
    source_map: SourceMap,
}

#[derive(Debug, Error, Diagnostic)]
pub enum CompileError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Source(#[from] SourceError),
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            entry: "main.dek".to_string(),
            source_map: SourceMap::new(FileLoader::new(".")),
        }
    }

    pub fn compile(mut self) -> Result<(), CompileError> {
        let mut cx = SourceContext::new();
        cx.extensions_mut().insert(EntryContext);

        let source = self.source_map.load(&cx, &self.entry)?;

        println!("Loaded: {:?}", source);

        Ok(())
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
