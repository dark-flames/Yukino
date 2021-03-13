#[cfg(test)]
use crate::query::ast::traits::FromPair;
#[cfg(test)]
use crate::query::grammar::Rule;
#[cfg(test)]
use std::fmt::Debug;

#[cfg(test)]
pub fn assert_parse_result<T: FromPair + Eq + Debug>(input: &'static str, result: T, rule: Rule) {
    use crate::query::grammar::Grammar;
    use pest::Parser;

    let pair = Grammar::parse(rule, input)
        .unwrap()
        .next()
        .unwrap();

    assert_eq!(T::from_pair(pair).unwrap(), result)
}