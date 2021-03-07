use crate::query::expr::Expression;
use crate::query::parse::{Error, Keyword, Parse, ParseBuffer, Token};
use crate::query::syntax::error::SyntaxError;


// todo: Reserved Words
#[allow(dead_code)]
pub struct Select {
    pub items: Vec<SelectItem>,
    pub from: From,
    pub where_clause: Option<Expression>,
    pub group_by: Option<Group>,
    pub order_by: Vec<OrderByItem>,
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
            buffer.pop_token()?;

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


pub struct OrderByItem {
    pub order_by: Expression,
    pub order: Order,
}

impl Parse for OrderByItem {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        Ok(OrderByItem {
            order_by: buffer.parse()?,
            order: buffer.parse()?
        })
    }
}

pub struct From {
    pub entity: String,
    pub alias: String
}

impl Parse for From {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self, Error> {
        let cursor = buffer.cursor();
        if let Token::Ident(entity) = buffer.pop_token()? {
            if let Token::Ident(alias) = buffer.pop_token()? {
                Ok(From {
                    entity: entity.to_string(),
                    alias: alias.to_string()
                })
            } else {
                Err(buffer.error_at(SyntaxError::UnexpectedAlias, cursor))
            }
        } else {
            Err(buffer.error_at(SyntaxError::UnexpectedEntityName, cursor))
        }
    }
}

#[allow(dead_code)]
pub struct Group {
    pub group_by: Expression,
    pub having: Expression,
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
