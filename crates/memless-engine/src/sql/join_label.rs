use sqlparser::ast::JoinOperator;

pub(crate) fn join_label(operator: &JoinOperator) -> &'static str {
    match operator {
        JoinOperator::LeftOuter(_) => "LEFT JOIN",
        JoinOperator::RightOuter(_) => "RIGHT JOIN",
        JoinOperator::FullOuter(_) => "FULL JOIN",
        JoinOperator::CrossJoin => "CROSS JOIN",
        _ => "this JOIN",
    }
}
