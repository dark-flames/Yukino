pub enum Expression {
    LogicalExpr(LogicalExpression),
    ComparisonExpr(ComparisonExpression),
    SubqueryExpr(SubqueryExpression),
    IdentExpr(IdentExpression),
    Function(Function)
}

pub enum LogicalExpression {
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Not(Box<Expression>)
}

pub enum ComparisonExpression {
    Eq(Box<Expression>, Box<Expression>),
    NotEq(Box<Expression>, Box<Expression>),
    GT(Box<Expression>, Box<Expression>),
    GTE(Box<Expression>, Box<Expression>),
    LT(Box<Expression>, Box<Expression>),
    LTE(Box<Expression>, Box<Expression>)
}

pub enum SubqueryExpression {
    In(Box<Expression>),
    Any(Box<Expression>),
    Some(Box<Expression>),
    ALL(Box<Expression>),
    Exists(Box<Expression>)
}

pub enum Function {
    Average(Box<Expression>),
    Max(Box<Expression>),
    Min(Box<Expression>),
    Count(Box<Expression>),
    Distinct(Box<Expression>), // todo move to other group?
    Abs(Box<Expression>),
    Contact(Vec<Expression>),
}

#[allow(dead_code)]
pub struct IdentExpression {
    segments: Vec<String>
}