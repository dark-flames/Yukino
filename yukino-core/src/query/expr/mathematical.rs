use crate::query::{ExpressionStructure, IdentExpression, Peekable, Value};
use proc_macro2::{Ident, Span};
use syn::parse::{Parse, ParseBuffer};
use syn::{token, Error, Ident as IdentMark, Token};

pub enum MathematicalExpression {
    BitInverse(Box<ExpressionStructure>),
    BitXor(Box<ExpressionStructure>, Box<ExpressionStructure>),
    Multi(Box<ExpressionStructure>, Box<ExpressionStructure>),
    Mod(Box<ExpressionStructure>, Box<ExpressionStructure>),
    Div(Box<ExpressionStructure>, Box<ExpressionStructure>),
    Add(Box<ExpressionStructure>, Box<ExpressionStructure>),
    Sub(Box<ExpressionStructure>, Box<ExpressionStructure>),
    BitLeftShift(Box<ExpressionStructure>, Box<ExpressionStructure>),
    BitRightShift(Box<ExpressionStructure>, Box<ExpressionStructure>),
    BitAnd(Box<ExpressionStructure>, Box<ExpressionStructure>),
    BitOr(Box<ExpressionStructure>, Box<ExpressionStructure>),
    GT(Box<ExpressionStructure>, Box<ExpressionStructure>),
    LT(Box<ExpressionStructure>, Box<ExpressionStructure>),
    GTE(Box<ExpressionStructure>, Box<ExpressionStructure>),
    LTE(Box<ExpressionStructure>, Box<ExpressionStructure>),
    EQ(Box<ExpressionStructure>, Box<ExpressionStructure>),
    NEQ(Box<ExpressionStructure>, Box<ExpressionStructure>),
    And(Box<ExpressionStructure>, Box<ExpressionStructure>),
    Or(Box<ExpressionStructure>, Box<ExpressionStructure>),
    Xor(Box<ExpressionStructure>, Box<ExpressionStructure>),
    Not(Box<ExpressionStructure>),
}

#[derive(Debug)]
pub enum BinaryOperator {
    BitXor(Token![^]),
    Multi(Token![*]),
    Mod(Token![%]),
    Div(Token![/]),
    Add(Token![+]),
    Sub(Token![-]),
    BitLeftShift(Token![<<]),
    BitRightShift(Token![>>]),
    BitAnd(Token![&]),
    BitOr(Token![|]),
    GT(Token![>]),
    LT(Token![<]),
    GTE(Token![>=]),
    LTE(Token![<=]),
    EQ(Token![==]),
    NEQ(Token![!=]),
    And,
    Or,
    Xor,
}

pub enum UnaryOperator {
    BitInverse(Token![~]),
    Not,
}

impl Parse for BinaryOperator {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        if input.peek(Token![^]) {
            input.parse().map(BinaryOperator::BitXor)
        } else if input.peek(Token![*]) {
            input.parse().map(BinaryOperator::Multi)
        } else if input.peek(Token![/]) {
            input.parse().map(BinaryOperator::Div)
        } else if input.peek(Token![%]) {
            input.parse().map(BinaryOperator::Mod)
        } else if input.peek(Token![+]) {
            input.parse().map(BinaryOperator::Add)
        } else if input.peek(Token![-]) {
            input.parse().map(BinaryOperator::Sub)
        } else if input.peek(Token![>>]) {
            input.parse().map(BinaryOperator::BitRightShift)
        } else if input.peek(Token![<<]) {
            input.parse().map(BinaryOperator::BitLeftShift)
        } else if input.peek(Token![&]) {
            input.parse().map(BinaryOperator::BitAnd)
        } else if input.peek(Token![|]) {
            input.parse().map(BinaryOperator::BitOr)
        } else if input.peek(Token![<=]) {
            input.parse().map(BinaryOperator::LTE)
        } else if input.peek(Token![>=]) {
            input.parse().map(BinaryOperator::GTE)
        } else if input.peek(Token![<]) {
            input.parse().map(BinaryOperator::LT)
        } else if input.peek(Token![>]) {
            input.parse().map(BinaryOperator::GT)
        } else if input.peek(Token![==]) {
            input.parse().map(BinaryOperator::EQ)
        } else if input.peek(Token![!=]) {
            input.parse().map(BinaryOperator::NEQ)
        } else if input.peek(IdentMark) {
            let ident: Ident = input.parse()?;
            let ident_str = ident.to_string().to_lowercase();
            match ident_str.as_str() {
                "and" => Ok(BinaryOperator::And),
                "or" => Ok(BinaryOperator::Or),
                "xor" => Ok(BinaryOperator::Xor),
                _ => Err(Error::new(
                    Span::call_site(),
                    "Unexpected binary logical operator",
                )),
            }
        } else if input.peek(Token![||]) {
            Ok(BinaryOperator::Or)
        } else if input.peek(Token![&&]) {
            Ok(BinaryOperator::And)
        } else {
            Err(Error::new(
                Span::call_site(),
                "Cannot parse into an binary operator",
            ))
        }
    }
}

impl Peekable for BinaryOperator {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(Token![^])
            | input.peek(Token![*])
            | input.peek(Token![/])
            | input.peek(Token![%])
            | input.peek(Token![+])
            | input.peek(Token![-])
            | input.peek(Token![>>])
            | input.peek(Token![<<])
            | input.peek(Token![&])
            | input.peek(Token![|])
            | input.peek(Token![<])
            | input.peek(Token![>])
            | input.peek(Token![<=])
            | input.peek(Token![>=])
            | input.peek(Token![==])
            | input.peek(Token![!=])
            | match input.fork().parse::<Ident>() {
                Ok(ident) => match ident.to_string().to_lowercase().as_str() {
                    "and" | "or" | "xor" => true,
                    _ => false,
                },
                _ => false,
            }
    }
}

impl Parse for UnaryOperator {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        if input.peek(Token![~]) {
            input.parse().map(UnaryOperator::BitInverse)
        } else if input.peek(IdentMark) {
            let ident: Ident = input.parse()?;
            let ident_str = ident.to_string().to_lowercase();
            match ident_str.as_str() {
                "not" => Ok(UnaryOperator::Not),
                _ => Err(Error::new(
                    Span::call_site(),
                    "Cannot parse into an Unary operator",
                )),
            }
        } else {
            Err(Error::new(
                Span::call_site(),
                "Cannot parse into an Unary operator",
            ))
        }
    }
}

impl Peekable for UnaryOperator {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        input.peek(Token![~]) || {
            let fork = input.fork();
            if fork.peek(IdentMark) {}
            match fork.parse::<IdentMark>() {
                Ok(ident) => match ident.to_string().to_lowercase().as_str() {
                    "not" => true,
                    _ => false,
                },
                _ => false,
            }
        }
    }
}

impl BinaryOperator {
    pub fn construct_expr(
        &self,
        left: ExpressionStructure,
        right: ExpressionStructure,
    ) -> MathematicalExpression {
        let left_box = Box::new(left);
        let right_box = Box::new(right);
        match self {
            BinaryOperator::BitXor(_) => MathematicalExpression::BitXor(left_box, right_box),
            BinaryOperator::Multi(_) => MathematicalExpression::Multi(left_box, right_box),
            BinaryOperator::Div(_) => MathematicalExpression::Div(left_box, right_box),
            BinaryOperator::Mod(_) => MathematicalExpression::Mod(left_box, right_box),
            BinaryOperator::Add(_) => MathematicalExpression::Add(left_box, right_box),
            BinaryOperator::Sub(_) => MathematicalExpression::Sub(left_box, right_box),
            BinaryOperator::BitLeftShift(_) => {
                MathematicalExpression::BitLeftShift(left_box, right_box)
            }
            BinaryOperator::BitRightShift(_) => {
                MathematicalExpression::BitRightShift(left_box, right_box)
            }
            BinaryOperator::BitAnd(_) => MathematicalExpression::BitAnd(left_box, right_box),
            BinaryOperator::BitOr(_) => MathematicalExpression::BitOr(left_box, right_box),
            BinaryOperator::GT(_) => MathematicalExpression::GT(left_box, right_box),
            BinaryOperator::LT(_) => MathematicalExpression::LT(left_box, right_box),
            BinaryOperator::GTE(_) => MathematicalExpression::GTE(left_box, right_box),
            BinaryOperator::LTE(_) => MathematicalExpression::LTE(left_box, right_box),
            BinaryOperator::EQ(_) => MathematicalExpression::EQ(left_box, right_box),
            BinaryOperator::NEQ(_) => MathematicalExpression::NEQ(left_box, right_box),
            BinaryOperator::Or => MathematicalExpression::Or(left_box, right_box),
            BinaryOperator::And => MathematicalExpression::And(left_box, right_box),
            BinaryOperator::Xor => MathematicalExpression::Xor(left_box, right_box),
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            BinaryOperator::BitXor(_) => Precedence::BitXor,
            BinaryOperator::Multi(_) | BinaryOperator::Div(_) | BinaryOperator::Mod(_) => {
                Precedence::Term
            }
            BinaryOperator::Add(_) | BinaryOperator::Sub(_) => Precedence::Add,
            BinaryOperator::BitLeftShift(_) | BinaryOperator::BitRightShift(_) => {
                Precedence::BitShift
            }
            BinaryOperator::BitAnd(_) => Precedence::BitAnd,
            BinaryOperator::BitOr(_) => Precedence::BitOr,
            BinaryOperator::GT(_)
            | BinaryOperator::LT(_)
            | BinaryOperator::GTE(_)
            | BinaryOperator::LTE(_)
            | BinaryOperator::EQ(_)
            | BinaryOperator::NEQ(_) => Precedence::Comparison,
            BinaryOperator::Or => Precedence::Or,
            BinaryOperator::And => Precedence::And,
            BinaryOperator::Xor => Precedence::Xor,
        }
    }
}

impl UnaryOperator {
    pub fn construct_expr(&self, expr: ExpressionStructure) -> MathematicalExpression {
        let boxed_expr = Box::new(expr);

        match self {
            UnaryOperator::BitInverse(_) => MathematicalExpression::BitInverse(boxed_expr),
            UnaryOperator::Not => MathematicalExpression::Not(boxed_expr),
        }
    }

    pub fn precedence(&self) -> Precedence {
        match self {
            UnaryOperator::BitInverse(_) => Precedence::BitInverse,
            UnaryOperator::Not => Precedence::Not,
        }
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Precedence {
    None,
    Or,
    Xor,
    And,
    Not,
    Comparison,
    BitOr,
    BitAnd,
    BitShift,
    Add,
    Term,
    BitXor,
    BitInverse,
}

impl Precedence {
    pub fn peek<'a>(input: &'a ParseBuffer<'a>) -> Option<Self> {
        if input.peek(Token![^]) {
            Some(Precedence::BitXor)
        } else if input.peek(Token![*]) | input.peek(Token![/]) | input.peek(Token![%]) {
            Some(Precedence::Term)
        } else if input.peek(Token![+]) | input.peek(Token![-]) {
            Some(Precedence::Add)
        } else if input.peek(Token![>>]) | input.peek(Token![<<]) {
            Some(Precedence::BitShift)
        } else if input.peek(Token![&]) {
            Some(Precedence::BitAnd)
        } else if input.peek(Token![|]) {
            Some(Precedence::BitOr)
        } else if input.peek(Token![<=])
            | input.peek(Token![>=])
            | input.peek(Token![<])
            | input.peek(Token![>])
            | input.peek(Token![==])
            | input.peek(Token![!=])
        {
            Some(Precedence::Comparison)
        } else if input.peek(IdentMark) {
            let ident: Ident = input.fork().parse().ok()?;
            let ident_str = ident.to_string().to_lowercase();
            match ident_str.as_str() {
                "or" => Some(Precedence::Or),
                "xor" => Some(Precedence::Xor),
                "and" => Some(Precedence::And),
                "not" => Some(Precedence::Not),
                _ => None,
            }
        } else if input.peek(Token![||]) {
            Some(Precedence::Or)
        } else if input.peek(Token![&&]) {
            Some(Precedence::And)
        } else {
            None
        }
    }
}

impl MathematicalExpression {
    pub fn parse_right_expression<'a>(
        input: &'a ParseBuffer<'a>,
        operator_precedence: Precedence,
    ) -> Result<ExpressionStructure, Error> {
        let result = if input.peek(token::Paren) {
            ExpressionStructure::parse_item(input)
        } else if UnaryOperator::peek(input) {
            let unary_operator: UnaryOperator = input.parse()?;
            let right = Self::parse_right_expression(input, unary_operator.precedence())?;
            Ok(ExpressionStructure::MathematicalExpr(
                unary_operator.construct_expr(right),
            ))
        } else if Value::peek(input) {
            let value = input.parse()?;
            Ok(ExpressionStructure::Value(value))
        } else if IdentExpression::peek(input) {
            let ident = input.parse()?;
            Ok(ExpressionStructure::IdentExpr(ident))
        } else {
            Err(Error::new(
                Span::call_site(),
                "Cannot parse into expression or unary operator",
            ))
        }?;

        let next_binary_operator_precedence = Precedence::peek(input);

        match next_binary_operator_precedence {
            Some(next_precedence) if next_precedence > operator_precedence => {
                let operator = input.parse::<BinaryOperator>()?;
                let next_expr = Self::parse_right_expression(input, operator.precedence())?;

                Ok(ExpressionStructure::MathematicalExpr(
                    operator.construct_expr(result, next_expr),
                ))
            }
            _ => Ok(result),
        }
    }

    pub fn parse_operator_and_right_expression<'a>(
        input: &'a ParseBuffer<'a>,
        left: ExpressionStructure,
    ) -> Result<ExpressionStructure, Error> {
        let mut result = left;

        while !input.is_empty() {
            let operator = if BinaryOperator::peek(input) {
                input.parse::<BinaryOperator>()?
            } else {
                break;
            };

            result = ExpressionStructure::MathematicalExpr(operator.construct_expr(
                result,
                Self::parse_right_expression(input, operator.precedence())?,
            ))
        }

        Ok(result)
    }

    pub fn parse_into_expression<'a>(
        input: &'a ParseBuffer<'a>,
    ) -> Result<ExpressionStructure, Error> {
        Self::parse_operator_and_right_expression(
            input,
            Self::parse_right_expression(input, Precedence::None)?,
        )
    }
}

impl Parse for MathematicalExpression {
    fn parse<'a>(input: &'a ParseBuffer<'a>) -> Result<Self, Error> {
        let left = Self::parse_right_expression(input, Precedence::None)?;

        match Self::parse_operator_and_right_expression(input, left)? {
            ExpressionStructure::MathematicalExpr(mathematical_expr) => Ok(mathematical_expr),
            _ => Err(Error::new(
                Span::call_site(),
                "Not a mathematical expression",
            )),
        }
    }
}

impl Peekable for MathematicalExpression {
    fn peek<'a>(input: &'a ParseBuffer<'a>) -> bool {
        Precedence::peek(input).is_some()
    }
}

#[test]
fn test_expr() {
    use crate::query::Function;
    use syn::Lit;
    let expr: ExpressionStructure = syn::parse_quote! {
        1 + @value * 10 > ~10 % (average(foo.bar) / 3)
    };

    if let ExpressionStructure::MathematicalExpr(MathematicalExpression::GT(gt_left, gt_right)) =
        expr
    {
        if let ExpressionStructure::MathematicalExpr(MathematicalExpression::Add(
            add_left,
            add_right,
        )) = *gt_left
        {
            if let ExpressionStructure::Value(Value::Lit(Lit::Int(lit))) = *add_left {
                assert_eq!(lit.base10_parse::<i32>().unwrap(), 1)
            } else {
                panic!("Add left value")
            }

            if let ExpressionStructure::MathematicalExpr(MathematicalExpression::Multi(
                multi_left,
                multi_right,
            )) = *add_right
            {
                if let ExpressionStructure::Value(Value::ExternalValue(ident)) = *multi_left {
                    assert_eq!(ident.to_string(), "value".to_string())
                } else {
                    panic!("multi left")
                }

                if let ExpressionStructure::Value(Value::Lit(Lit::Int(lit))) = *multi_right {
                    assert_eq!(lit.base10_parse::<i32>().unwrap(), 10);
                } else {
                    panic!("multi right")
                }
            }
        } else {
            panic!("Add");
        }

        if let ExpressionStructure::MathematicalExpr(MathematicalExpression::Mod(
            mod_left,
            mod_right,
        )) = *gt_right
        {
            if let ExpressionStructure::MathematicalExpr(MathematicalExpression::BitInverse(
                inverse,
            )) = *mod_left
            {
                if let ExpressionStructure::Value(Value::Lit(Lit::Int(lit))) = *inverse {
                    assert_eq!(lit.base10_parse::<i32>().unwrap(), 10)
                } else {
                    panic!("Inverse value");
                }
            } else {
                panic!("Inverse")
            }

            if let ExpressionStructure::MathematicalExpr(MathematicalExpression::Div(
                div_left,
                div_right,
            )) = *mod_right
            {
                if let ExpressionStructure::Function(Function::Average(aver)) = *div_left {
                    if let ExpressionStructure::IdentExpr(ident) = *aver {
                        assert_eq!(ident.segments, vec!["foo".to_string(), "bar".to_string()]);
                    } else {
                        panic!("Ident");
                    }
                } else {
                    panic!("Function");
                }

                if let ExpressionStructure::Value(Value::Lit(Lit::Int(lit))) = *div_right {
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

#[test]
fn test_logical() {
    use syn::Lit;
    let expr: ExpressionStructure = syn::parse_quote! {
        a.ratio > 20 AND (NOT b.ratio <= @value)
    };

    if let ExpressionStructure::MathematicalExpr(MathematicalExpression::And(and_left, and_right)) =
        expr
    {
        if let ExpressionStructure::MathematicalExpr(MathematicalExpression::GT(
            gt_left,
            gt_right,
        )) = *and_left
        {
            if let ExpressionStructure::IdentExpr(ident) = *gt_left {
                assert_eq!(ident.segments, vec!["a".to_string(), "ratio".to_string()]);
            } else {
                panic!("A ident");
            }

            if let ExpressionStructure::Value(Value::Lit(Lit::Int(lit))) = *gt_right {
                assert_eq!(lit.base10_parse::<i32>().unwrap(), 20);
            } else {
                panic!("lit");
            }
        }

        if let ExpressionStructure::MathematicalExpr(MathematicalExpression::Not(not)) = *and_right
        {
            if let ExpressionStructure::MathematicalExpr(MathematicalExpression::LTE(
                lte_left,
                lte_right,
            )) = *not
            {
                if let ExpressionStructure::IdentExpr(ident) = *lte_left {
                    assert_eq!(ident.segments, vec!["b".to_string(), "ratio".to_string()]);
                } else {
                    panic!("B ident");
                }

                if let ExpressionStructure::Value(Value::ExternalValue(ident)) = *lte_right {
                    assert_eq!(ident.to_string(), "value".to_string());
                } else {
                    panic!("external value");
                }
            } else {
                panic!("lte");
            }
        } else {
            panic!("not");
        }
    } else {
        panic!("and");
    }
}
