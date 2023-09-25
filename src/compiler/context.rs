use super::{interner::Interner, symbol::Symbol};

pub struct Context {
    pub symbol_interner: Interner<Symbol>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            symbol_interner: Interner::new(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
