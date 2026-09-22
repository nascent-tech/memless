use super::outside::outside;
use memless_domain::query::Op;
use memless_domain::QueryRefusal;
use sqlparser::ast::BinaryOperator;

pub(crate) fn binary_op(op: &BinaryOperator) -> Result<Op, QueryRefusal> {
    match op {
        BinaryOperator::Eq => Ok(Op::Eq),
        BinaryOperator::NotEq => Ok(Op::Ne),
        BinaryOperator::Lt => Ok(Op::Lt),
        BinaryOperator::LtEq => Ok(Op::Le),
        BinaryOperator::Gt => Ok(Op::Gt),
        BinaryOperator::GtEq => Ok(Op::Ge),
        _ => Err(outside("this operator")),
    }
}
