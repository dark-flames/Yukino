use crate::query::expr::logical::LogicalExpression;
use crate::query::expr::mathematical::MathematicalExpression;
use crate::query::{Function, IdentExpression, SubqueryExpression, Value, Peekable, UnaryOperator, BinaryOperator};
use syn::parse::{Parse, ParseBuffer};
use syn::{Error, parenthesized};
use syn::token::Paren;
use proc_macro2::Span;

pub enum Expression {
    MathematicalExpr(MathematicalExpression),
    LogicalExpr(LogicalExpression),
    SubqueryExpr(SubqueryExpression),
    IdentExpr(IdentExpression),
    Function(Function),
    Value(Value),
}

impl Parse for Expression {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let mut result = Self::parse_item(input)?;

        while !input.is_empty() {
            if BinaryOperator::peek(input) {
                result = MathematicalExpression::parse_operator_and_right_expression(input, result)?
            } else {
                return Err(Error::new(Span::call_site(), "Unexpected expression part"))
            }
        };

        Ok(result)
    }
}

#[allow(dead_code)]
impl Expression {
    pub fn parse_item<'a>(input: &'a ParseBuffer<'a>) -> Result<Expression, Error> {
        if input.peek(Paren) {
            let content;

            parenthesized!(content in input);

            content.parse()
        } else if Function::peek(input) {
            input.parse().map(Expression::Function)
        } else if IdentExpression::peek(input) {
            input.parse().map(Expression::IdentExpr)
        } else if Value::peek(input) {
            input.parse().map(Expression::Value)
        } else if UnaryOperator::peek(input) {
            input.parse().map(Expression::MathematicalExpr)
        } else {
            Err(Error::new(Span::call_site(), "Unexpected expression item"))
        }
    }
}

#[test]
fn test_expr() {
    use syn::Lit;
    let expr: Expression = syn::parse_quote! {
        1 + @value * 10 > ~10 % (foo.bar / 3)
    };

    if let Expression::MathematicalExpr(MathematicalExpression::GT(gt_left, gt_right)) = expr {
        if let Expression::MathematicalExpr(MathematicalExpression::Add(add_left, add_right)) =
        *gt_left
        {
            if let Expression::Value(Value::Lit(Lit::Int(lit))) = *add_left {
                assert_eq!(lit.base10_parse::<i32>().unwrap(), 1)
            } else {
                panic!("Add left value")
            }

            if let Expression::MathematicalExpr(MathematicalExpression::Multi(
                                                    multi_left,
                                                    multi_right,
                                                )) = *add_right
            {
                if let Expression::Value(Value::ExternalValue(ident)) = *multi_left {
                    assert_eq!(ident.to_string(), "value".to_string())
                } else {
                    panic!("multi left")
                }

                if let Expression::Value(Value::Lit(Lit::Int(lit))) = *multi_right {
                    assert_eq!(lit.base10_parse::<i32>().unwrap(), 10);
                } else {
                    panic!("multi right")
                }
            }
        } else {
            panic!("Add");
        }

        if let Expression::MathematicalExpr(MathematicalExpression::Mod(mod_left, mod_right)) =
        *gt_right
        {
            if let Expression::MathematicalExpr(MathematicalExpression::BitInverse(inverse)) =
            *mod_left
            {
                if let Expression::Value(Value::Lit(Lit::Int(lit))) = *inverse {
                    assert_eq!(lit.base10_parse::<i32>().unwrap(), 10)
                } else {
                    panic!("Inverse value");
                }
            } else {
                panic!("Inverse")
            }

            if let Expression::MathematicalExpr(MathematicalExpression::Div(div_left, div_right)) =
            *mod_right
            {
                if let Expression::IdentExpr(ident) = *div_left {
                    assert_eq!(ident.segments, vec!["foo".to_string(), "bar".to_string()]);
                } else {
                    panic!("Ident");
                }

                if let Expression::Value(Value::Lit(Lit::Int(lit))) = *div_right {
                    assert_eq!(lit.base10_parse::<i32>().unwrap(), 3);
                } else {
                    panic!("Div right");
                }
            } else {
                panic!("Div");
            }
        } else {
            panic!("Mod");
        }
    } else {
        panic!("GT")
    }
}
