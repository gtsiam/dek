use std::fmt;

use crate::vm::value::Value;

use super::{interner::Interned, symbol::Symbol};

pub enum ExprKind {
    Literal(Value),
    Identifier(Identifier),
    Call(Call),
    Todo,
}

impl fmt::Debug for ExprKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(v) => fmt::Debug::fmt(&v, f),
            Self::Identifier(v) => fmt::Debug::fmt(&v, f),
            Self::Call(v) => fmt::Debug::fmt(&v, f),
            Self::Todo => write!(f, "Todo"),
        }
    }
}

pub struct Expr {
    pub kind: ExprKind,
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <ExprKind as fmt::Debug>::fmt(&self.kind, f)
    }
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Self { kind }
    }

    pub fn todo() -> Self {
        Self::new(ExprKind::Todo)
    }

    pub fn literal(lit: Value) -> Self {
        Self::new(ExprKind::Literal(lit))
    }

    pub fn call(fun: Expr, arg: Expr) -> Self {
        Self::new(ExprKind::Call(Call::new(fun, arg)))
    }

    pub fn bin_op(fun: Expr, left: Expr, right: Expr) -> Self {
        Expr::call(Expr::call(fun, left), right)
    }
}

pub struct Call {
    pub fun: Box<Expr>,
    pub arg: Box<Expr>,
}

impl fmt::Debug for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("(")?;
        fmt::Debug::fmt(&self.fun, f)?;
        f.write_str(" ")?;
        fmt::Debug::fmt(&self.arg, f)?;
        f.write_str(")")
    }
}

impl Call {
    pub fn new(fun: Expr, arg: Expr) -> Self {
        Self {
            fun: Box::new(fun),
            arg: Box::new(arg),
        }
    }
}

pub struct Identifier {
    pub name: Interned<Symbol>,
}

impl fmt::Debug for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.name, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static_assertions::assert_impl_all!(Expr: Send, Sync);
}
