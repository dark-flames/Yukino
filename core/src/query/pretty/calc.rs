use crate::query::ast::error::{SyntaxError, SyntaxErrorWithPos};
use crate::query::ast::{
    Binary, BinaryOperator, Boolean, ColumnIdent, DeleteQuery, Expr, Float, FromClause,
    GroupByClause, InsertQuery, Integer, JoinClause, JoinOn, Literal, Locatable, Null,
    OrderByClause, Query, SelectClause, SelectQuery, SetClause, Unary, UpdateQuery, ValueItem,
};
use crate::query::type_check::TypeKind;
use std::cmp::Ordering;

pub trait Calc: Locatable {
    fn calc(&mut self) -> Result<Option<Literal>, SyntaxErrorWithPos> {
        Ok(None)
    }
}

impl Calc for Expr {
    fn calc(&mut self) -> Result<Option<Literal>, SyntaxErrorWithPos> {
        match self {
            Expr::Literal(lit) => lit.calc(),
            Expr::ColumnIdent(ident) => ident.calc(),
            Expr::Binary(binary) => binary.calc(),
            Expr::Unary(unary) => unary.calc(),
            _ => unimplemented!(),
        }
    }
}

impl Calc for Literal {
    fn calc(&mut self) -> Result<Option<Literal>, SyntaxErrorWithPos> {
        if let Literal::External(_) = self {
            Ok(None)
        } else {
            Ok(Some(self.clone()))
        }
    }
}

impl Calc for ColumnIdent {}

impl Calc for Binary {
    fn calc(&mut self) -> Result<Option<Literal>, SyntaxErrorWithPos> {
        let location = self.location();
        let self_operator = self.operator;

        let left_result = self.left.calc()?;
        let right_result = self.right.calc()?;

        if let Some(left) = &left_result {
            self.left = Box::new(Expr::Literal(left.clone()));
        }
        if let Some(right) = &right_result {
            self.right = Box::new(Expr::Literal(right.clone()));
        }

        match (left_result, right_result) {
            (Some(Literal::Integer(left)), Some(Literal::Integer(right))) => {
                let left_value: i128 = left.value.parse().map_err(|_| {
                    left.location()
                        .error(SyntaxError::CannotParseIntoInteger(left.value.clone()))
                })?;

                let right_value: i128 = right.value.parse().map_err(|_| {
                    right
                        .location()
                        .error(SyntaxError::CannotParseIntoInteger(right.value.clone()))
                })?;
                let result_value = match self_operator {
                    BinaryOperator::BitXor => left_value ^ right_value,
                    BinaryOperator::Multi => left_value * right_value,
                    BinaryOperator::Div => {
                        return Ok(Some(Literal::Float(Float {
                            value: (left_value as f64 / right_value as f64).to_string(),
                            location,
                        })))
                    }
                    BinaryOperator::Mod => left_value % right_value,
                    BinaryOperator::Plus => left_value + right_value,
                    BinaryOperator::Minus => left_value - right_value,
                    BinaryOperator::LeftShift => left_value << right_value,
                    BinaryOperator::RightShift => left_value >> right_value,
                    BinaryOperator::BitAnd => left_value & right_value,
                    BinaryOperator::BitOr => left_value | right_value,
                    operator if operator.is_cmp() => {
                        return Ok(Some(Literal::Boolean(Boolean {
                            value: match operator {
                                BinaryOperator::Bte => left_value >= right_value,
                                BinaryOperator::Lte => left_value <= right_value,
                                BinaryOperator::Neq => left_value != right_value,
                                BinaryOperator::Bt => left_value > right_value,
                                BinaryOperator::Lt => left_value < right_value,
                                BinaryOperator::Eq => left_value == right_value,
                                _ => unreachable!(),
                            },
                            location,
                        })))
                    }
                    op => {
                        return Err(location.error(SyntaxError::UnimplementedOperationForType(
                            format!("{:?}", op),
                            "integer".to_string(),
                        )))
                    }
                };

                Ok(Some(Literal::Integer(Integer {
                    value: result_value.to_string(),
                    location,
                })))
            }
            (Some(Literal::Float(left)), Some(Literal::Float(right))) => {
                let left_value: f64 = left.value.parse().map_err(|_| {
                    left.location()
                        .error(SyntaxError::CannotParseIntoInteger(left.value.clone()))
                })?;

                let right_value: f64 = right.value.parse().map_err(|_| {
                    right
                        .location()
                        .error(SyntaxError::CannotParseIntoInteger(right.value.clone()))
                })?;

                let result_value = match self_operator {
                    BinaryOperator::Multi => left_value * right_value,
                    BinaryOperator::Div => left_value / right_value,
                    BinaryOperator::Mod => left_value % right_value,
                    BinaryOperator::Plus => left_value + right_value,
                    BinaryOperator::Minus => left_value - right_value,
                    operator if operator.is_cmp() => {
                        return Ok(Some(Literal::Boolean(Boolean {
                            value: match operator {
                                BinaryOperator::Bte => matches!(
                                    left_value.partial_cmp(&right_value),
                                    Some(Ordering::Equal) | Some(Ordering::Greater)
                                ),
                                BinaryOperator::Lte => matches!(
                                    left_value.partial_cmp(&right_value),
                                    Some(Ordering::Equal) | Some(Ordering::Less)
                                ),
                                BinaryOperator::Neq => matches!(
                                    left_value.partial_cmp(&right_value),
                                    Some(Ordering::Less) | Some(Ordering::Greater)
                                ),
                                BinaryOperator::Bt => matches!(
                                    left_value.partial_cmp(&right_value),
                                    Some(Ordering::Greater)
                                ),
                                BinaryOperator::Lt => matches!(
                                    left_value.partial_cmp(&right_value),
                                    Some(Ordering::Less)
                                ),
                                BinaryOperator::Eq => matches!(
                                    left_value.partial_cmp(&right_value),
                                    Some(Ordering::Equal)
                                ),
                                _ => unreachable!(),
                            },
                            location,
                        })))
                    }
                    op => {
                        return Err(location.error(SyntaxError::UnimplementedOperationForType(
                            format!("{:?}", op),
                            "float".to_string(),
                        )))
                    }
                };

                Ok(Some(Literal::Float(Float {
                    value: result_value.to_string(),
                    location,
                })))
            }
            (Some(Literal::Boolean(left)), Some(Literal::Boolean(right))) => {
                Ok(Some(Literal::Boolean(Boolean {
                    value: match self_operator {
                        BinaryOperator::BitAnd => left.value & right.value,
                        BinaryOperator::BitOr => left.value | right.value,
                        BinaryOperator::BitXor => left.value ^ right.value,
                        BinaryOperator::Neq => left.value != right.value,
                        BinaryOperator::Eq => left.value == right.value,
                        BinaryOperator::And => left.value && right.value,
                        BinaryOperator::Xor => left.value ^ right.value,
                        BinaryOperator::Or => left.value || right.value,
                        operator => {
                            return Err(location.error(SyntaxError::UnimplementedOperationForType(
                                format!("{:?}", operator),
                                "Bool".to_string(),
                            )))
                        }
                    },
                    location,
                })))
            }
            (Some(Literal::Null(_)), _) | (_, Some(Literal::Null(_))) => {
                Ok(Some(Literal::Null(Null { location })))
            }
            (Some(Literal::String(_)), Some(Literal::String(_))) => {
                Err(location.error(SyntaxError::UnimplementedOperationForType(
                    format!("{:?}", self_operator),
                    "fstring".to_string(),
                )))
            }
            (Some(left), Some(right)) => Err(right.location().error(SyntaxError::TypeError(
                TypeKind::from(&left).to_string(),
                TypeKind::from(&right).to_string(),
            ))),
            _ => Ok(None),
        }
    }
}

impl Calc for Unary {
    fn calc(&mut self) -> Result<Option<Literal>, SyntaxErrorWithPos> {
        let right_result = self.right.calc()?;
        if let Some(right) = right_result {
            match right {
                Literal::Integer(_) => Ok(None),
                Literal::Boolean(boolean) => Ok(Some(Literal::Boolean(Boolean {
                    value: !boolean.value,
                    location: self.location(),
                }))),
                Literal::Null(_) => Ok(Some(Literal::Null(Null {
                    location: self.location(),
                }))),
                _ => Err(self
                    .location()
                    .error(SyntaxError::UnimplementedOperationForType(
                        format!("{:?}", self.operator),
                        "fstring".to_string(),
                    ))),
            }
        } else {
            Ok(None)
        }
    }
}

pub trait CalcExpr {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos>;
}

impl CalcExpr for Query {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        match self {
            Query::Select(select) => select.calc_expr(),
            Query::Delete(delete) => delete.calc_expr(),
            Query::Update(update) => update.calc_expr(),
            Query::Insert(insert) => insert.calc_expr(),
        }
    }
}

impl CalcExpr for SelectQuery {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        self.select_clause.calc_expr()?;
        self.from.calc_expr()?;
        if let Some(where_clause) = &mut self.where_clause {
            if let Some(result) = where_clause.calc()? {
                self.where_clause = Some(Expr::Literal(result));
            }
        }

        if let Some(group_clause) = &mut self.group_by_clause {
            group_clause.calc_expr()?;
        }

        if let Some(order_by_clause) = &mut self.order_by_clause {
            order_by_clause.calc_expr()?;
        }

        Ok(())
    }
}

impl CalcExpr for SelectClause {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        for (expr, _) in self.items.iter_mut() {
            if let Some(new_expr) = expr.calc()? {
                *expr = Expr::Literal(new_expr)
            }
        }

        Ok(())
    }
}

impl CalcExpr for FromClause {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        for join in self.join.iter_mut() {
            join.calc_expr()?;
        }

        Ok(())
    }
}

impl CalcExpr for JoinClause {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        if let JoinClause::JoinOn(join_on) = self {
            join_on.calc_expr()
        } else {
            Ok(())
        }
    }
}

impl CalcExpr for JoinOn {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        if let Some(result) = self.on.calc()? {
            self.on = Expr::Literal(result);
        };

        Ok(())
    }
}

impl CalcExpr for GroupByClause {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        if let Some(result) = self.by.calc()? {
            self.by = Expr::Literal(result);
        };

        if let Some(having_clause) = &mut self.having {
            if let Some(result) = having_clause.calc()? {
                self.having = Some(Expr::Literal(result));
            }
        }

        Ok(())
    }
}

impl CalcExpr for OrderByClause {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        for (expr, _) in self.items.iter_mut() {
            if let Some(result) = expr.calc()? {
                *expr = Expr::Literal(result);
            };
        }

        Ok(())
    }
}

impl CalcExpr for DeleteQuery {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        self.from.calc_expr()?;
        if let Some(where_clause) = &mut self.where_clause {
            if let Some(result) = where_clause.calc()? {
                self.where_clause = Some(Expr::Literal(result));
            }
        }

        Ok(())
    }
}

impl CalcExpr for UpdateQuery {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        if let Some(where_clause) = &mut self.where_clause {
            if let Some(result) = where_clause.calc()? {
                self.where_clause = Some(Expr::Literal(result));
            }
        }

        self.set_clause.calc_expr()
    }
}

impl CalcExpr for SetClause {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        for (_, value) in self.items.iter_mut() {
            value.calc_expr()?;
        }

        Ok(())
    }
}

impl CalcExpr for ValueItem {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        if let ValueItem::Expr(expr) = self {
            if let Some(result) = expr.calc()? {
                *expr = Expr::Literal(result);
            }
        }

        Ok(())
    }
}

impl CalcExpr for InsertQuery {
    fn calc_expr(&mut self) -> Result<(), SyntaxErrorWithPos> {
        for expr in self.values.iter_mut() {
            if let Some(new_expr) = expr.calc()? {
                *expr = Expr::Literal(new_expr)
            }
        }

        Ok(())
    }
}

#[test]
fn test_calc() {
    use crate::query::ast::*;
    use crate::query::grammar::*;
    use pest::Parser;

    let mut expr1 = Expr::from_pair(
        Grammar::parse(Rule::expr, "9 + 10 * 5 = 59")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();

    let location = Location::pos(0);

    assert_eq!(
        expr1.calc().unwrap(),
        Some(Literal::Boolean(Boolean {
            value: true,
            location
        }))
    );

    let mut expr2 = Expr::from_pair(
        Grammar::parse(Rule::expr, "(15 + 10) / 5")
            .unwrap()
            .next()
            .unwrap(),
    )
    .unwrap();

    assert_eq!(
        expr2.calc().unwrap(),
        Some(Literal::Float(Float {
            value: "5".to_string(),
            location
        }))
    );
}
