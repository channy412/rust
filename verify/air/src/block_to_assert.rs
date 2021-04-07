use crate::ast::{BinaryOp, Const, Expr, ExprX, LogicalOp, Query, QueryX, Stmt, StmtX};
use std::rc::Rc;

pub fn stmt_to_expr(stmt: &Stmt, pred: Expr) -> Expr {
    match &**stmt {
        StmtX::Assume(expr) => {
            // wp((assume Q), P) = Q ==> P
            Rc::new(ExprX::Binary(BinaryOp::Implies, expr.clone(), pred))
        }
        StmtX::Assert(span, expr) => {
            // wp((assert Q), P) = Q /\ P
            let assertion = Rc::new(ExprX::LabeledAssertion(span.clone(), expr.clone()));
            Rc::new(ExprX::Logical(LogicalOp::And, Rc::new(Box::new([assertion, pred]))))
        }
        StmtX::Block(stmts) => {
            // wp((s1; s2), P) = wp(s1, wp(s2, P))
            let mut p = pred;
            for stmt in stmts.iter().rev() {
                p = stmt_to_expr(stmt, p);
            }
            p
        }
    }
}

pub fn block_to_assert(stmt: &Stmt) -> Expr {
    let tru = Rc::new(ExprX::Const(Const::Bool(true)));
    stmt_to_expr(&stmt, tru)
}

pub fn lower_query(query: &Query) -> Query {
    let expr = crate::block_to_assert::block_to_assert(&query.assertion);
    let assertion = Rc::new(StmtX::Assert(Rc::new(None), expr));
    Rc::new(QueryX { local: query.local.clone(), assertion })
}
