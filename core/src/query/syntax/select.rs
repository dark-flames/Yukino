use crate::query::expr::Expression;
use crate::query::helper::Peekable;
use proc_macro2::Ident;
use syn::parse::{Parse, ParseBuffer};
use syn::{Error, Ident as IdentMark, Token};

pub enum SelectItem {
    All,
    Expr(Expression),
    Alias { expr: Expression, alias: Ident },
}

impl Peekable for SelectItem {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(Token![*]) || Expression::peek(input)
    }
}

impl Parse for SelectItem {
    fn parse(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;

            Ok(SelectItem::All)
        } else if let Ok(expr) = input.parse() {
            if input.peek(Token![as]) {
                input.parse::<Token![as]>()?;
                Ok(SelectItem::Alias {
                    expr,
                    alias: input.parse()?,
                })
            } else if input.peek(IdentMark) {
                let ident = input.parse::<Ident>()?;

                if ident.to_string().to_lowercase() == "as" {
                    Ok(SelectItem::Alias {
                        expr,
                        alias: input.parse()?,
                    })
                } else {
                    Err(input.error("Cannot parse into SelectItem"))
                }
            } else {
                Ok(SelectItem::Expr(expr))
            }
        } else {
            Err(input.error("Cannot parse into SelectItem"))
        }
    }
}
