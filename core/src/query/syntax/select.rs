use crate::query::expr::Expression;
use crate::query::parse::Ident;

pub enum SelectItem {
    All,
    Expr(Expression),
    Alias { expr: Expression, alias: Ident },
}
