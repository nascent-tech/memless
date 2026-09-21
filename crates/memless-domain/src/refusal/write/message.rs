use super::WriteRefusal;
use super::WriteRefusal::{ColumnCountMismatch, ColumnRepeated, DiskWriteFailed};

pub(crate) fn write_message(refusal: &WriteRefusal) -> String {
    if let ColumnCountMismatch { table, columns, values } = refusal {
        return format!("INSERT into {table:?} names {columns} columns but gives {values} values");
    }
    if let ColumnRepeated { table, column } = refusal {
        return format!("{column:?} repeated in the write to {table:?}");
    }
    if let DiskWriteFailed { path, kind } = refusal {
        return format!("cannot write file {path:?}: {kind}");
    }
    String::new()
}
