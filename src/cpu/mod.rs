pub mod logical;
use core::cell::RefCell;

use alloc::sync::Arc;

use self::logical::LogicalCpuId;
use crate::context::switch::ContextSwitchPercpu;

// 存储每个CPU的变量信息，可应用于多核CPU的情况
pub struct PercpuBlock {
    // A unique immutable number that identifies the current CPU - used for scheduling
    pub cpu_id: LogicalCpuId,

    // 上下文管理
    pub switch_internals: ContextSwitchPercpu,
    pub current_addrsp: RefCell<Option<Arc<AddrSpaceWrapper>>>,
}

impl PercpuBlock {
    pub fn init(cpu_id: LogicalCpuId) -> Self {
        Self { cpu_id }
    }
}
