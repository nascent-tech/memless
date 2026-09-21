use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, PoisonError};

use memless_engine::Base;

use crate::status::MemlessHandle;

#[derive(Default)]
struct Table {
    next: MemlessHandle,
    bases: HashMap<MemlessHandle, Base>,
}

static INSTANCES: LazyLock<Mutex<Table>> = LazyLock::new(|| Mutex::new(Table::default()));

pub(crate) fn store(base: Base) -> MemlessHandle {
    let mut table = INSTANCES.lock().unwrap_or_else(PoisonError::into_inner);
    table.next = table.next.saturating_add(1);
    let handle = table.next;
    table.bases.insert(handle, base);
    handle
}

pub(crate) fn discard(handle: MemlessHandle) {
    let mut table = INSTANCES.lock().unwrap_or_else(PoisonError::into_inner);
    let base = table.bases.remove(&handle);
    drop(table);
    drop(base);
}

pub(crate) fn with_base<R>(handle: MemlessHandle, body: impl FnOnce(&Base) -> R) -> Option<R> {
    let table = INSTANCES.lock().unwrap_or_else(PoisonError::into_inner);
    table.bases.get(&handle).map(body)
}
