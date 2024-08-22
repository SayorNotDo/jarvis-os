/*
    上下文切换（context switch）
*/

pub mod context;
pub mod memory;
pub mod process;
pub mod switch;

use alloc::{collections::BTreeSet, sync::Arc};
use spin::{Once, RwLock, RwLockReadGuard, RwLockWriteGuard};
use spinning_top::RwSpinlock;

use self::process::Process;

use self::context::Context;

// use crate::cpu::PercpuBlock;

static KMAIN_PROCESS: Once<Arc<RwLock<Process>>> = Once::new();

// 可调度上下文的弱引用集合，上下文文件描述符（context file descriptors）是唯一的强引用
static CONTEXTS: RwLock<BTreeSet<ContextRef>> = RwLock::new(BTreeSet::new());

pub struct ContextRef(pub Arc<RwSpinlock<Context>>);

pub fn init() {}

impl ContextRef {
    pub fn upgrade(&self) -> Option<Arc<RwSpinlock<Context>>> {
        Some(Arc::clone(&self.0))
    }
}

impl Ord for ContextRef {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Ord::cmp(&Arc::as_ptr(&self.0), &Arc::as_ptr(&other.0))
    }
}

impl PartialOrd for ContextRef {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl PartialEq for ContextRef {
    fn eq(&self, other: &Self) -> bool {
        Ord::cmp(self, other) == core::cmp::Ordering::Equal
    }
}

impl Eq for ContextRef {}

pub fn contexts() -> RwLockReadGuard<'static, BTreeSet<ContextRef>> {
    CONTEXTS.read()
}

pub fn contexts_mut() -> RwLockWriteGuard<'static, BTreeSet<ContextRef>> {
    CONTEXTS.write()
}

// pub fn current() -> Arc<RwSpinlock<Context>> {}
