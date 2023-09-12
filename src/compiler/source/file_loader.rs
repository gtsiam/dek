use std::{
    io,
    path::{Path, PathBuf},
};

use miette::Diagnostic;
use thiserror::Error;

use crate::compiler::source::EntryContext;

use super::{Source, SourceContext, SourceLoader};

pub struct FileLoader {
    /// The base path from where all file accesses will be resolved.
    base_path: PathBuf,
}

impl FileLoader {
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_owned(),
        }
    }
}

/// The context attached to files loaded with a [`FileLoader`]
#[derive(Debug)]
pub struct FileContext {
    path: PathBuf,
}

#[derive(Debug, Error, Diagnostic)]
pub enum FileLoaderError {
    #[error("Invalid source context")]
    #[diagnostic(help("Files can only be loaded from the context of other files"))]
    InvalidContext,

    #[error("I/O error for `{path}`")]
    IO {
        #[source]
        err: io::Error,
        path: PathBuf,
    },
}

impl SourceLoader for FileLoader {
    type Key = PathBuf;
    type Error = FileLoaderError;

    fn resolve(&mut self, cx: &SourceContext, name: &str) -> Result<Self::Key, Self::Error> {
        // Find the base path relative to which we will resolve the import.
        let mut base = None;

        if let Some(cx) = cx.extensions().get::<FileContext>() {
            base = Some(&cx.path);
        }

        if let Some(_cx) = cx.extensions().get::<EntryContext>() {
            base = Some(&self.base_path);
        }

        let base = base.ok_or(FileLoaderError::InvalidContext)?;
        Ok(base.join(name))
    }

    fn load(&mut self, _cx: &SourceContext, key: &Self::Key) -> Result<Source, Self::Error> {
        let contents = std::fs::read_to_string(key).map_err(|err| FileLoaderError::IO {
            err,
            path: key.clone(),
        })?;

        Ok(Source::new(contents))
    }
}
