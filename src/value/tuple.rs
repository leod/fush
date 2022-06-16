use crate::{Po, Value};

use super::{Binding, Constructible};

impl<U, V> Binding for (U, V)
where
    U: Binding,
    V: Binding,
{
    type Type = (Po<U>, Po<V>);
}

impl<U, V> Value for (U, V)
where
    U: Constructible,
    V: Constructible,
{
    fn ty() -> crate::lang::Ty {
        todo!()
    }

    fn from_ident(ident: crate::lang::Ident) -> Self {
        todo!()
    }

    fn expr(&self) -> crate::lang::Expr {
        todo!()
    }
}

impl<U, V> Constructible for (U, V)
where
    U: Constructible,
    V: Constructible,
{
    fn from_trace(trace: super::Trace) -> Self {
        todo!()
    }
}
