use crate::query::expr::Expression;
use crate::query::parse::{Error, Keyword, Parse, ParseBuffer, Peek, Symbol, Token};
use crate::query::syntax::error::SyntaxError;

// todo: Reserved Words
// todo: join
#[derive(Debug, Eq, PartialEq)]
pub struct Select {
    pub items: Vec<SelectItem>,
    pub from: From,
    pub where_clause: Option<Expression>,
    pub group: Option<Group>,
    pub order_by: Vec<OrderByItem>,
}

impl Parse for Select {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        buffer.parse_token(Token::Keyword(Keyword::Select))?;

        let mut items = vec![];

        loop {
            let item = buffer.parse()?;

            items.push(item);

            if !buffer.peek_token(Token::Symbol(Symbol::Comma)) {
                break;
            }

            buffer.parse_token(Token::Symbol(Symbol::Comma))?;
        }

        buffer.parse_token(Token::Keyword(Keyword::From))?;

        let from = buffer.parse()?;

        let where_clause = if buffer.peek_token(Token::Keyword(Keyword::Where)) {
            buffer.parse_token(Token::Keyword(Keyword::Where))?;

            Some(buffer.parse()?)
        } else {
            None
        };

        let group = if buffer.peek_token(Token::Keyword(Keyword::GroupBy)) {
            buffer.parse_token(Token::Keyword(Keyword::GroupBy))?;

            let group_by = buffer.parse()?;

            let having = if buffer.peek_token(Token::Keyword(Keyword::Having)) {
                buffer.parse_token(Token::Keyword(Keyword::Having))?;

                Some(buffer.parse()?)
            } else {
                None
            };

            Some(Group { group_by, having })
        } else {
            None
        };

        let order_by = if buffer.peek_token(Token::Keyword(Keyword::OrderBy)) {
            buffer.parse_token(Token::Keyword(Keyword::OrderBy))?;
            let mut items = vec![];

            loop {
                let item = buffer.parse()?;

                items.push(item);

                if !buffer.peek_token(Token::Symbol(Symbol::Comma)) {
                    break;
                }

                buffer.parse_token(Token::Symbol(Symbol::Comma))?;
            }

            items
        } else {
            vec![]
        };

        Ok(Select {
            items,
            from,
            where_clause,
            group,
            order_by,
        })
    }
}

impl Peek for Select {
    fn peek(buffer: &ParseBuffer) -> bool {
        buffer.peek_token(Token::Keyword(Keyword::Select))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct SelectItem {
    pub expr: Expression,
    pub alias: Option<String>,
}

impl Parse for SelectItem {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let expr = buffer.parse()?;

        let alias = if let Some(Token::Keyword(Keyword::As)) = buffer.get_token() {
            buffer.parse_token(Token::Keyword(Keyword::As))?;

            Some(if let Some(Token::Ident(ident)) = buffer.get_token() {
                let result = Ok(ident.to_string());
                buffer.pop_token()?;

                result
            } else {
                Err(buffer.error_head(SyntaxError::UnexpectedAlias))
            }?)
        } else {
            None
        };

        Ok(SelectItem { expr, alias })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Order {
    Desc,
    Asc,
}

impl Parse for Order {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let result = match buffer.get_token() {
            Some(Token::Keyword(Keyword::Asc)) => Ok(Order::Asc),
            Some(Token::Keyword(Keyword::Desc)) => Ok(Order::Desc),
            _ => Err(buffer.error_head(SyntaxError::CannotParseIntoOrder)),
        }?;

        buffer.pop_token()?;

        Ok(result)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct OrderByItem {
    pub order_by: Expression,
    pub order: Order,
}

impl Parse for OrderByItem {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        Ok(OrderByItem {
            order_by: buffer.parse()?,
            order: buffer.parse()?,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct From {
    pub entity: String,
    pub alias: String,
}

impl Parse for From {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let cursor = buffer.cursor();
        if let Token::Ident(entity) = buffer.pop_token()? {
            if let Token::Ident(alias) = buffer.pop_token()? {
                Ok(From {
                    entity: entity.to_string(),
                    alias: alias.to_string(),
                })
            } else {
                Err(buffer.error_at(SyntaxError::UnexpectedAlias, cursor))
            }
        } else {
            Err(buffer.error_at(SyntaxError::UnexpectedEntityName, cursor))
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Group {
    pub group_by: Expression,
    pub having: Option<Expression>,
}

#[test]
fn test_select_item() {
    use crate::query::expr::{DatabaseIdent, FunctionCall};
    use crate::query::parse::TokenStream;
    use std::str::FromStr;

    let tokens1 = TokenStream::from_str("sum(t.count) as s").unwrap();

    let result1: SelectItem = tokens1.parse().unwrap();

    assert_eq!(
        result1,
        SelectItem {
            expr: Expression::Function(FunctionCall {
                ident: "sum".to_string(),
                parameters: vec![Expression::Ident(DatabaseIdent {
                    segments: vec!["t".to_string(), "count".to_string()]
                })]
            }),
            alias: Some("s".to_string())
        }
    )
}

#[test]
fn test_select() {
    use crate::query::expr::{BinaryExpression, DatabaseIdent, FunctionCall, Literal};
    use crate::query::parse::TokenStream;
    use std::str::FromStr;

    let query = TokenStream::from_str(
        "SELECT count(*), sum(t.count) AS sum FROM test t WHERE t.id <= 100 GROUP BY t.ty having t.ty != 3 ORDER BY t.ty DESC "
    ).unwrap();

    let result: Select = query.parse().unwrap();

    assert_eq!(
        result,
        Select {
            items: vec![
                SelectItem {
                    expr: Expression::Function(FunctionCall {
                        ident: "count".to_string(),
                        parameters: vec![Expression::Ident(DatabaseIdent {
                            segments: vec!["*".to_string()]
                        })]
                    }),
                    alias: None
                },
                SelectItem {
                    expr: Expression::Function(FunctionCall {
                        ident: "sum".to_string(),
                        parameters: vec![Expression::Ident(DatabaseIdent {
                            segments: vec!["t".to_string(), "count".to_string()]
                        })]
                    }),
                    alias: Some("sum".to_string())
                },
            ],
            from: From {
                entity: "test".to_string(),
                alias: "t".to_string()
            },
            where_clause: Some(Expression::Binary(BinaryExpression::LTE(
                Box::new(Expression::Ident(DatabaseIdent {
                    segments: vec!["t".to_string(), "id".to_string()]
                })),
                Box::new(Expression::Literal(Literal::Int(100))),
            ))),
            group: Some(Group {
                group_by: Expression::Ident(DatabaseIdent {
                    segments: vec!["t".to_string(), "ty".to_string()]
                }),
                having: Some(Expression::Binary(BinaryExpression::NEQ(
                    Box::new(Expression::Ident(DatabaseIdent {
                        segments: vec!["t".to_string(), "ty".to_string()]
                    })),
                    Box::new(Expression::Literal(Literal::Int(3))),
                )))
            }),
            order_by: vec![OrderByItem {
                order_by: Expression::Ident(DatabaseIdent {
                    segments: vec!["t".to_string(), "ty".to_string()]
                }),
                order: Order::Desc
            }]
        }
    );
}
