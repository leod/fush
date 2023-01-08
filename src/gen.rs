mod scope_form;
mod simplified_expr;
mod struct_registry;
mod var_form;

pub mod glsl;

pub use scope_form::{Scope, ScopeForm, VarInit};
pub use simplified_expr::{SimplifiedExpr, VarId};
pub use struct_registry::StructRegistry;
pub use var_form::VarForm;

use std::rc::Rc;

use crate::dag::Expr;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct ExprKey(*const Expr);

impl<'a> From<&'a Rc<Expr>> for ExprKey {
    fn from(value: &'a Rc<Expr>) -> Self {
        ExprKey(&**value as *const _)
    }
}
