use core::{fmt::Error, sync::atomic::AtomicU32};

use alloc::sync::Arc;
use spin::RwLock;

#[derive(Debug)]
pub struct AddrSpaceWrapper {
    inner: RwLock<AddrSpace>,
    pub tlb_ack: AtomicU32,
}
// impl AddrSpaceWrapper {
//     pub fn new() -> Result<Arc<Self>> {
//         Arc::try_new(Self {
//             inner: RwLock::new(AddrSpace::new()?),
//             tlb_ack: AtomicU32::new(0),
//         })
//         .map_err(|_| Error::new(ENOMEM))
//     }
// }

#[derive(Debug)]
pub struct AddrSpace {
    pub table: Table,
}

// impl AddrSpace {
//     pub fn new() -> Result<Self> {
//         Ok(Self {})
//     }
// }

#[derive(Debug)]
pub struct Table {}
