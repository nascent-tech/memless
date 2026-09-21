use std::collections::HashMap;
use std::sync::{LazyLock, Mutex, PoisonError};

use memless_engine::Instance;

use crate::status::MemlessHandle;

#[derive(Default)]
struct Table {
    next: MemlessHandle,
    instances: HashMap<MemlessHandle, Instance>,
}

static INSTANCES: LazyLock<Mutex<Table>> = LazyLock::new(|| Mutex::new(Table::default()));

pub(crate) fn store(instance: Instance) -> MemlessHandle {
    let mut table = INSTANCES.lock().unwrap_or_else(PoisonError::into_inner);
    table.next = table.next.saturating_add(1);
    let handle = table.next;
    table.instances.insert(handle, instance);
    handle
}

pub(crate) fn discard(handle: MemlessHandle) {
    let mut table = INSTANCES.lock().unwrap_or_else(PoisonError::into_inner);
    let instance = table.instances.remove(&handle);
    drop(table);
    drop(instance);
}

pub(crate) fn with_instance<R>(handle: MemlessHandle, body: impl FnOnce(&Instance) -> R) -> Option<R> {
    let table = INSTANCES.lock().unwrap_or_else(PoisonError::into_inner);
    table.instances.get(&handle).map(body)
}

pub(crate) fn with_instance_mut<R>(handle: MemlessHandle, body: impl FnOnce(&mut Instance) -> R) -> Option<R> {
    let mut table = INSTANCES.lock().unwrap_or_else(PoisonError::into_inner);
    table.instances.get_mut(&handle).map(body)
}
