use crate::query::expr::binary::{BinaryExpression, BinaryOperator};
use crate::query::expr::error::ExprParseError;
use crate::query::expr::function::FunctionCall;
use crate::query::expr::ident::DatabaseIdent;
use crate::query::expr::literal::Literal;
use crate::query::expr::precedence::Precedence;
use crate::query::expr::unary::{UnaryExpression, UnaryOperator};
use crate::query::parse::{Error, Parse, ParseBuffer, Symbol, Token};

// todo: Subquery
#[derive(Debug, Eq, PartialEq)]
pub enum Expression {
    Unary(UnaryExpression),
    Binary(BinaryExpression),
    Ident(DatabaseIdent),
    Function(FunctionCall),
    Literal(Literal),
}

impl Parse for Expression {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let head = buffer.cursor();

        Expression::parse_item_with_precedence(buffer, Precedence::None)?
            .ok_or_else(|| buffer.error_at(ExprParseError::CannotParseIntoExpression, head))
    }
}

impl Expression {
    pub fn parse_item_with_precedence(
        buffer: &mut ParseBuffer,
        precedence: Precedence,
    ) -> Result<Option<Self>, Error> {
        let head = buffer.cursor();
        let mut result = if buffer.peek_token(Token::Symbol(Symbol::ParenLeft)) {
            buffer.pop_token()?;
            let offset = buffer
                .get_tokens()
                .iter()
                .position(|token| matches!(token, Token::Symbol(Symbol::ParenRight)))
                .ok_or_else(|| buffer.error_at(ExprParseError::CannotFindRightParen, head))?;

            let mut new_buffer = buffer.split_at(offset);

            let expr = new_buffer.parse::<Expression>()?;

            assert!(new_buffer.is_empty());

            buffer.pop_token()?;

            expr
        } else if precedence <= Precedence::Lit && buffer.peek::<Literal>() {
            Expression::Literal(buffer.parse()?)
        } else if precedence <= Precedence::FunctionCall && buffer.peek::<FunctionCall>() {
            Expression::Function(buffer.parse()?)
        } else if precedence <= Precedence::Ident && buffer.peek::<DatabaseIdent>() {
            Expression::Ident(buffer.parse()?)
        } else if buffer.peek::<UnaryOperator>() {
            let operator = UnaryOperator::peek_operator(buffer)?;

            if precedence <= operator.precedence() {
                Expression::Unary(buffer.parse()?)
            } else {
                return Ok(None);
            }
        } else {
            return Err(buffer.error_at(ExprParseError::CannotParseIntoExpression, head));
        };

        while let Ok(operator) = BinaryOperator::peek_operator(buffer) {
            if precedence <= operator.precedence() {
                result = Expression::Binary(BinaryExpression::parse_right_side(buffer, result)?);
            } else {
                break;
            }
        }

        Ok(Some(result))
    }
}

#[test]
fn test_expr() {
    use crate::query::parse::TokenStream;
    use std::str::FromStr;

    let token_stream = TokenStream::from_str(
        "-5 + (1 + ~2) * 10.0 <= test(table.column.*, \"やりますねぇ\", false)",
    )
    .unwrap();

    let result: Expression = token_stream.parse().unwrap();

    assert_eq!(
        result,
        Expression::Binary(BinaryExpression::LTE(
            Box::new(Expression::Binary(BinaryExpression::Add(
                Box::new(Expression::Literal(Literal::Int(-5))),
                Box::new(Expression::Binary(BinaryExpression::Multi(
                    Box::new(Expression::Binary(BinaryExpression::Add(
                        Box::new(Expression::Literal(Literal::Int(1))),
                        Box::new(Expression::Unary(UnaryExpression::BitInverse(Box::new(
                            Expression::Literal(Literal::Int(2))
                        ))))
                    ))),
                    Box::new(Expression::Literal(Literal::Float(10.0)))
                )))
            ))),
            Box::new(Expression::Function(FunctionCall {
                ident: "test".to_string(),
                parameters: vec![
                    Expression::Ident(DatabaseIdent {
                        segments: vec!["table".to_string(), "column".to_string(), "*".to_string()]
                    }),
                    Expression::Literal(Literal::Str("やりますねぇ".to_string())),
                    Expression::Literal(Literal::Bool(false)),
                ]
            }))
        ))
    )
}
