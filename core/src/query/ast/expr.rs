use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::func::FunctionCall;
use crate::query::ast::ident::ColumnIdent;
use crate::query::ast::traits::{FromPair, Locatable, QueryPair};
use crate::query::ast::{Literal, Location};
use crate::query::grammar::Rule;

type BoxedExpr = Box<Expr>;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    FunctionCall(FunctionCall),
    ColumnIdent(ColumnIdent),
    Unary(Unary),
    Binary(Binary),
}

impl FromPair for Expr {
    fn from_pair(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();

        match pair.as_rule() {
            Rule::expr => Binary::from_pair(pair),
            _ => Err(location.error(SyntaxError::UnexpectedPair("expr"))),
        }
    }
}

impl Locatable for Expr {
    fn location(&self) -> Location {
        match self {
            Expr::Literal(lit) => lit.location(),
            Expr::FunctionCall(func) => func.location(),
            Expr::ColumnIdent(ident) => ident.location(),
            Expr::Unary(e) => e.location(),
            Expr::Binary(e) => e.location(),
        }
    }
}

impl Expr {
    fn parse_factory(pair: QueryPair) -> Result<Self, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();
        match pair.as_rule() {
            Rule::expr_factor => {
                let inner = pair
                    .into_inner()
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedPair("expr_factor")))?;

                match inner.as_rule() {
                    Rule::literal => Literal::from_pair(inner).map(Expr::Literal),
                    Rule::function_call => FunctionCall::from_pair(inner).map(Expr::FunctionCall),
                    Rule::column_ident => ColumnIdent::from_pair(inner).map(Expr::ColumnIdent),
                    Rule::expr => Self::from_pair(inner),
                    _ => {
                        Err(Location::from(&inner)
                            .error(SyntaxError::UnexpectedPair("expr_factor")))
                    }
                }
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("expr_factor"))),
        }
    }
}

fn expression_from_pair(
    pair: QueryPair,
    binary: bool,
    factory: bool,
) -> Result<Expr, SyntaxErrorWithPos> {
    if binary {
        Binary::from_pair(pair)
    } else if factory {
        Expr::parse_factory(pair)
    } else {
        Unary::from_pair(pair)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum BinaryOperator {
    BitXor,
    Multi,
    Div,
    Mod,
    Plus,
    Minus,
    LeftShift,
    RightShift,
    BitAnd,
    BitOr,
    Bte,
    Lte,
    Neq,
    Bt,
    Lt,
    Eq,
    And,
    Xor,
    Or,
}

impl BinaryOperator {
    pub fn from_rule(rule: Rule) -> Option<Self> {
        match rule {
            Rule::plus => Some(BinaryOperator::Plus),
            Rule::minus => Some(BinaryOperator::Plus),
            Rule::multi => Some(BinaryOperator::Multi),
            Rule::div => Some(BinaryOperator::Div),
            Rule::modulo => Some(BinaryOperator::Mod),
            Rule::left_shift => Some(BinaryOperator::LeftShift),
            Rule::right_shift => Some(BinaryOperator::RightShift),
            Rule::bit_and => Some(BinaryOperator::BitAnd),
            Rule::bit_or => Some(BinaryOperator::BitOr),
            Rule::bit_xor => Some(BinaryOperator::BitXor),
            Rule::less_than => Some(BinaryOperator::Lt),
            Rule::less_than_or_equal => Some(BinaryOperator::Lte),
            Rule::bigger_than => Some(BinaryOperator::Bt),
            Rule::bigger_than_or_equal => Some(BinaryOperator::Bte),
            Rule::equal => Some(BinaryOperator::Eq),
            Rule::not_equal => Some(BinaryOperator::Neq),
            Rule::bool_and => Some(BinaryOperator::And),
            Rule::bool_or => Some(BinaryOperator::Or),
            Rule::bool_xor => Some(BinaryOperator::Xor),
            _ => None,
        }
    }

    pub fn compare_operator() -> Vec<Self> {
        use BinaryOperator::*;
        vec![Bte, Lte, Neq, Bt, Lt, Eq]
    }

    pub fn term_operator() -> Vec<Self> {
        use BinaryOperator::*;
        vec![Multi, Div, Mod]
    }

    pub fn add_operator() -> Vec<Self> {
        use BinaryOperator::*;
        vec![Plus, Minus]
    }

    pub fn bit_shift_operator() -> Vec<Self> {
        use BinaryOperator::*;
        vec![LeftShift, RightShift]
    }
}

#[derive(Clone, Debug)]
pub struct Binary {
    pub operator: BinaryOperator,
    pub left: BoxedExpr,
    pub right: BoxedExpr,
    pub location: Location,
}

impl Binary {
    fn handle_ast(
        pair: QueryPair,
        allowed_operators: Vec<BinaryOperator>,
        next_binary: bool,
    ) -> Result<Expr, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();
        let mut inner = pair.into_inner();
        let mut result = expression_from_pair(
            inner
                .next()
                .ok_or_else(|| location.error(SyntaxError::UnexpectedExpr))?,
            next_binary,
            false,
        )?;

        while let Some(inner_pair) = inner.next() {
            let operator = BinaryOperator::from_rule(inner_pair.as_rule())
                .ok_or_else(|| location.error(SyntaxError::UnexpectedExpr))?;

            if !allowed_operators.contains(&operator) {
                break;
            }

            let item = expression_from_pair(
                inner
                    .next()
                    .ok_or_else(|| location.error(SyntaxError::UnexpectedExpr))?,
                next_binary,
                false,
            )?;

            let new_location = Location::span(result.location().start(), item.location().end());

            result = Expr::Binary(Binary {
                operator,
                left: Box::new(result),
                right: Box::new(item),
                location: new_location,
            })
        }

        Ok(result)
    }
}

impl PartialEq for Binary {
    fn eq(&self, other: &Self) -> bool {
        self.operator == other.operator && self.left == other.left && self.right == other.right
    }
}

impl Eq for Binary {}

impl FromPair<Expr> for Binary {
    fn from_pair(pair: QueryPair) -> Result<Expr, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();

        match pair.as_rule() {
            Rule::expr => Self::handle_ast(pair, vec![BinaryOperator::Or], true),
            Rule::xor_expr => Self::handle_ast(pair, vec![BinaryOperator::Xor], true),
            Rule::and_expr => Self::handle_ast(pair, vec![BinaryOperator::And], false),
            Rule::cmp_expr => Self::handle_ast(pair, BinaryOperator::compare_operator(), true),
            Rule::bit_or_expr => Self::handle_ast(pair, vec![BinaryOperator::BitOr], true),
            Rule::bit_and_expr => Self::handle_ast(pair, vec![BinaryOperator::BitAnd], true),
            Rule::bit_shift_expr => {
                Self::handle_ast(pair, BinaryOperator::bit_shift_operator(), true)
            }
            Rule::add_expr => Self::handle_ast(pair, BinaryOperator::add_operator(), true),
            Rule::term_expr => Self::handle_ast(pair, BinaryOperator::term_operator(), true),
            Rule::bit_xor_expr => Self::handle_ast(pair, vec![BinaryOperator::BitXor], false),
            _ => Err(location.error(SyntaxError::UnexpectedPair("expr"))),
        }
    }
}

impl Locatable for Binary {
    fn location(&self) -> Location {
        self.location
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum UnaryOperator {
    Not,
    BitReverse,
}

impl UnaryOperator {
    pub fn from_rule(rule: Rule) -> Option<Self> {
        match rule {
            Rule::bit_reverse => Some(UnaryOperator::BitReverse),
            Rule::bool_not => Some(UnaryOperator::Not),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub right: BoxedExpr,
    pub location: Location,
}

impl PartialEq for Unary {
    fn eq(&self, other: &Self) -> bool {
        self.operator == other.operator && self.right == other.right
    }
}

impl Eq for Unary {}

impl FromPair<Expr> for Unary {
    fn from_pair(pair: QueryPair) -> Result<Expr, SyntaxErrorWithPos> {
        let location: Location = (&pair).into();

        match pair.as_rule() {
            Rule::not_expr => {
                let mut inner = pair.into_inner();

                if inner.clone().count() == 1 {
                    expression_from_pair(
                        inner
                            .next()
                            .ok_or_else(|| location.error(SyntaxError::UnexpectedExpr))?,
                        true,
                        false,
                    )
                } else if let Some(Some(UnaryOperator::Not)) = inner
                    .next()
                    .map(|inner_pair| UnaryOperator::from_rule(inner_pair.as_rule()))
                {
                    Ok(Expr::Unary(Unary {
                        operator: UnaryOperator::Not,
                        right: Box::new(Self::from_pair(inner.next().ok_or_else(|| {
                            location.error(SyntaxError::UnexpectedPair("not_expr"))
                        })?)?),
                        location,
                    }))
                } else {
                    Err(location.error(SyntaxError::UnexpectedPair("bool_not")))
                }
            }
            Rule::bit_reverse_expr => {
                let mut inner = pair.into_inner();

                if inner.clone().count() == 1 {
                    expression_from_pair(
                        inner
                            .next()
                            .ok_or_else(|| location.error(SyntaxError::UnexpectedExpr))?,
                        false,
                        true,
                    )
                } else if let Some(Some(UnaryOperator::BitReverse)) = inner
                    .next()
                    .map(|inner_pair| UnaryOperator::from_rule(inner_pair.as_rule()))
                {
                    Ok(Expr::Unary(Unary {
                        operator: UnaryOperator::BitReverse,
                        right: Box::new(Self::from_pair(inner.next().ok_or_else(|| {
                            location.error(SyntaxError::UnexpectedPair("bit_reverse_expr"))
                        })?)?),
                        location,
                    }))
                } else {
                    Err(location.error(SyntaxError::UnexpectedPair("bit_reverse")))
                }
            }
            _ => Err(location.error(SyntaxError::UnexpectedPair("unary_expr"))),
        }
    }
}

impl Locatable for Unary {
    fn location(&self) -> Location {
        self.location
    }
}

#[test]
fn test_expr() {
    use crate::query::ast::*;
    use crate::query::ast::helper::assert_parse_result;

    let location = Location::pos(0);

    assert_parse_result(
        "1 + 1 * 10 = 11",
        Expr::Binary(Binary {
            operator: BinaryOperator::Eq,
            left: Box::new(Expr::Binary(Binary {
                operator: BinaryOperator::Plus,
                left: Box::new(Expr::Literal(Literal::Integer(Integer {
                    value: 1,
                    location,
                }))),
                right: Box::new(Expr::Binary(Binary {
                    operator: BinaryOperator::Multi,
                    left: Box::new(Expr::Literal(Literal::Integer(Integer {
                        value: 1,
                        location,
                    }))),
                    right: Box::new(Expr::Literal(Literal::Integer(Integer {
                        value: 10,
                        location,
                    }))),
                    location,
                })),
                location,
            })),
            right: Box::new(Expr::Literal(Literal::Integer(Integer {
                value: 11,
                location,
            }))),
            location,
        }),
        Rule::expr
    );

    assert_parse_result(
        "(1 + 1) * 10 != 11",
        Expr::Binary(Binary {
            operator: BinaryOperator::Neq,
            left: Box::new(Expr::Binary(Binary {
                operator: BinaryOperator::Multi,
                left: Box::new(Expr::Binary(Binary {
                    operator: BinaryOperator::Plus,
                    left: Box::new(Expr::Literal(Literal::Integer(Integer {
                        value: 1,
                        location,
                    }))),
                    right: Box::new(Expr::Literal(Literal::Integer(Integer {
                        value: 1,
                        location,
                    }))),
                    location,
                })),
                right: Box::new(Expr::Literal(Literal::Integer(Integer {
                    value: 10,
                    location,
                }))),
                location,
            })),
            right: Box::new(Expr::Literal(Literal::Integer(Integer {
                value: 11,
                location,
            }))),
            location,
        }),
        Rule::expr
    );

    assert_parse_result(
        "column.a >= 11.1 AND NOT test(column.\"b\" + 10, Null) OR false",
        Expr::Binary(Binary {
            operator: BinaryOperator::Or,
            left: Box::new(Expr::Binary(Binary {
                operator: BinaryOperator::And,
                left: Box::new(Expr::Binary(Binary {
                    operator: BinaryOperator::Bte,
                    left: Box::new(Expr::ColumnIdent(ColumnIdent {
                        segments: vec!["column".to_string(), "a".to_string()],
                        location,
                    })),
                    right: Box::new(Expr::Literal(Literal::Float(Float {
                        value: 11.1,
                        location,
                    }))),
                    location,
                })),
                right: Box::new(Expr::Unary(Unary {
                    operator: UnaryOperator::Not,
                    right: Box::new(Expr::FunctionCall(FunctionCall {
                        ident: "test".to_string(),
                        parameters: vec![
                            Expr::Binary(Binary {
                                operator: BinaryOperator::Plus,
                                left: Box::new(Expr::ColumnIdent(ColumnIdent {
                                    segments: vec!["column".to_string(), "b".to_string()],
                                    location,
                                })),
                                right: Box::new(Expr::Literal(Literal::Integer(Integer {
                                    value: 10,
                                    location,
                                }))),
                                location,
                            }),
                            Expr::Literal(Literal::Null(Null { location })),
                        ],
                        location,
                    })),
                    location,
                })),
                location,
            })),
            right: Box::new(Expr::Literal(Literal::Boolean(Boolean {
                value: false,
                location,
            }))),
            location,
        }),
        Rule::expr
    )
}
