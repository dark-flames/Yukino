use crate::query::expr::Expression;
use crate::query::parse::{Error, Ident, Keyword, Parse, ParseBuffer, Token};
use crate::query::syntax::error::SyntaxError;

#[allow(dead_code)]
pub struct Select {
    pub items: Vec<SelectItem>,
    pub from: From,
    pub where_clause: Option<Expression>,
    pub group_by: Option<Group>,
    pub order_by: Vec<Order>,
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
                Err(buffer.error_head(SyntaxError::ExpectAnAlias))
            }?)
        } else {
            None
        };

        Ok(SelectItem { expr, alias })
    }
}

#[allow(dead_code)]
pub enum Order {
    Desc,
    Asc,
}

#[allow(dead_code)]
pub struct OrderByItem {
    order_by: Expression,
    order: Order,
}

#[allow(dead_code)]
pub enum From {
    Entity(Ident),
    Alias { entity: Ident, alias: Ident },
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
