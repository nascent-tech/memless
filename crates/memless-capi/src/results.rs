use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, PoisonError};

use super::prepared::Prepared;
use crate::status::MemlessResult;

#[derive(Default)]
struct Table {
    next: MemlessResult,
    results: HashMap<MemlessResult, Prepared>,
}

static RESULTS: LazyLock<Mutex<Table>> = LazyLock::new(|| Mutex::new(Table::default()));

pub(crate) fn store_result(prepared: Prepared) -> MemlessResult {
    let mut table = RESULTS.lock().unwrap_or_else(PoisonError::into_inner);
    table.next = table.next.saturating_add(1);
    let result = table.next;
    table.results.insert(result, prepared);
    result
}

pub(crate) fn discard_result(result: MemlessResult) {
    let mut table = RESULTS.lock().unwrap_or_else(PoisonError::into_inner);
    let prepared = table.results.remove(&result);
    drop(table);
    drop(prepared);
}

pub(crate) fn with_result<R>(result: MemlessResult, body: impl FnOnce(&Prepared) -> R) -> Option<R> {
    let table = RESULTS.lock().unwrap_or_else(PoisonError::into_inner);
    table.results.get(&result).map(body)
}
