use lalrpop_util::lalrpop_mod;
use miette::Diagnostic;
use thiserror::Error;

lalrpop_mod!(grammar, "/compiler/grammar.rs");

use self::source::{EntryContext, FileLoader, SourceContext, SourceError, SourceMap};

pub mod ast;
pub mod source;

pub struct Compiler {
    /// The source map.
    source_map: SourceMap,
}

#[derive(Debug, Error, Diagnostic)]
pub enum CompileError {
    #[error("Error loading source")]
    Source(#[from] SourceError),
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            source_map: SourceMap::new(FileLoader::new(".")),
        }
    }

    pub fn compile(&mut self, entry: impl AsRef<str>) -> Result<(), CompileError> {
        self._compile(entry.as_ref())
    }

    fn _compile(&mut self, entry: &str) -> Result<(), CompileError> {
        let mut source_cx = SourceContext::new();
        source_cx.extensions_mut().insert(EntryContext);

        let source = self.source_map.load(&source_cx, entry)?;

        let ast = grammar::ExprParser::new().parse(source.lexer());

        let _ = dbg!(ast);

        Ok(())
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}
