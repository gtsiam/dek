use core::fmt;
use std::sync::Arc;

use malachite::Rational;

pub struct Value {
    pub kind: Arc<ValueKind>,
}

impl Value {
    pub fn new(kind: ValueKind) -> Self {
        Self {
            kind: Arc::new(kind),
        }
    }

    pub fn boolean(value: bool) -> Self {
        Self::new(ValueKind::Boolean(Boolean { value }))
    }

    pub fn number(value: Rational) -> Self {
        Self::new(ValueKind::Number(Number { value }))
    }

    pub fn null() -> Self {
        Self::new(ValueKind::Null)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.kind, f)
    }
}

pub enum ValueKind {
    Null,
    Boolean(Boolean),
    Number(Number),
    Function(Function),
}

impl fmt::Debug for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Null => write!(f, "null"),
            Self::Boolean(v) => fmt::Debug::fmt(&v, f),
            Self::Number(v) => fmt::Debug::fmt(&v, f),
            Self::Function(v) => fmt::Debug::fmt(&v, f),
        }
    }
}

pub struct Number {
    pub value: Rational,
}

impl fmt::Debug for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}

pub struct Function {
    pub body: Box<dyn Fn(&Value) -> Value + Send + Sync>,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function>")
    }
}

pub struct Boolean {
    pub value: bool,
}

impl fmt::Debug for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.value, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_impl_all!(Value: Send, Sync);
}
