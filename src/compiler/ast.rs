use core::fmt;
use std::sync::Arc;

use malachite::Rational;

#[derive(Debug)]
pub enum ExprKind {
    NumberLit(NumberLit),
    FunctionCall(FunctionCall),
    Function(Function),
}

#[derive(Clone)]
pub struct Expr {
    pub kind: Arc<ExprKind>,
}

impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <ExprKind as fmt::Debug>::fmt(&self.kind, f)
    }
}

impl Expr {
    pub fn new(kind: ExprKind) -> Self {
        Self {
            kind: Arc::new(kind),
        }
    }

    pub fn bin_op(fun: Expr, left: Expr, right: Expr) -> Self {
        Expr::new(ExprKind::FunctionCall(FunctionCall {
            fun: Expr::new(ExprKind::FunctionCall(FunctionCall { fun, arg: left })),
            arg: right,
        }))
    }
}

#[derive(Debug)]
pub struct NumberLit {
    pub value: Rational,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub fun: Expr,
    pub arg: Expr,
}

pub struct Function {
    pub name: String,
    pub body: Box<dyn Fn(&Expr) -> Expr + Send + Sync>,
}

impl Function {
    pub fn identity() -> Expr {
        Expr::new(ExprKind::Function(Self {
            name: "identity".to_string(),
            body: Box::new(|expr| expr.clone()),
        }))
    }
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function {}>", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const _: () = {
        const fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Expr>();
    };
}
