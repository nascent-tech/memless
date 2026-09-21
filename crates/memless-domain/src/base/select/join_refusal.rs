use super::other_name::other_name;
use super::relation_side::relation_side;
use crate::base::table::Table;
use crate::query::Join;
use crate::refusal::QueryRefusal;
use crate::refusal::QueryRefusal::JoinNotRelation;

pub(crate) fn join_refusal(from: &Table, joined: &Table, join: &Join) -> QueryRefusal {
    let side = relation_side(&join.left, &join.right);
    let table = side.table.clone().unwrap_or_else(|| from.name.clone());
    let target = other_name(from, joined, &table);
    JoinNotRelation { table, column: side.column.clone(), target }
}
