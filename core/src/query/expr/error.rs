use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExprParseError {
    #[error("Cannot parse token stream into DatabaseIdent")]
    CannotParseIntoIdent,
    #[error("Unexpected '*' in database ident")]
    UnexpectedAny,
    #[error("Cannot parse token stream into Literal")]
    CannotParseIntoLit,
    #[error("Cannot parse token stream into Function")]
    CannotParseIntoFunction,
    #[error("Cannot parse \"{0}\" into string")]
    CannotParseFloat(String),
    #[error("Expect an unary operator here")]
    CannotParseIntoUnaryOperator,
    #[error("Expect a binary operator here")]
    CannotParseIntoBinaryOperator,
    #[error("Expect some token")]
    CannotParseIntoExpression,
    #[error("Unmatched parenthesis missing ')'")]
    CannotFindRightParen,
}
