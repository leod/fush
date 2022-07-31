use std::{collections::BTreeMap, rc::Rc};

use super::Expr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExprId(usize);

/*impl fmt::Debug for ExprId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", get(*self))
    }
}*/

pub struct ExprReg {
    next_id: ExprId,
    exprs: BTreeMap<ExprId, Rc<Expr>>,
}

impl Default for ExprReg {
    fn default() -> Self {
        Self {
            next_id: ExprId(0),
            exprs: BTreeMap::new(),
        }
    }
}

impl ExprReg {
    pub fn new() -> Self {
        ExprReg::default()
    }

    pub fn insert(&mut self, expr: Expr) -> ExprId {
        let expr = Rc::new(expr);

        let id = self.next_id;
        self.next_id.0 += 1;

        self.exprs.insert(id, expr.clone());

        id
    }

    pub fn get(&self, id: ExprId) -> Rc<Expr> {
        self.exprs.get(&id).unwrap().clone()
    }

    pub fn len(&self) -> usize {
        self.exprs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.exprs.is_empty()
    }
}
