#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct LogicalCpuId(u32);

impl LogicalCpuId {
    pub const BSP: Self = Self::new(0);

    pub const fn new(inner: u32) -> Self {
        Self(inner)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}
