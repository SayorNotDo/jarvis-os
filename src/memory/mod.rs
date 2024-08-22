use core::fmt::Error;

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

#[derive(Debug)]
pub struct Enomem;

impl From<Enomem> for Error {
    fn from(_: Enomem) -> Self {
        Self::new(ENOMEM)
    }
}

/*
    结构体属性 `repr(transparent)` 确保结构体在内存中表示与它唯一的字段类型完全相同
    其允许创建类型安全的包装类型而不会引入额外的内存开销或改变数据的表示方式
*/
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct FrameCount(usize);

pub struct FrameUsage {
    used: FrameCount,
    total: FrameCount,
}

impl FrameUsage {
    pub fn new(used: FrameCount, total: FrameCount) -> Self {
        Self { used, total }
    }

    pub fn used(&self) -> FrameCount {
        self.used
    }
}

// 页帧分配器，从bootloader中的内存映射中返回可用的页帧
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /*
        非安全函数：初始化操作
        从内存映射中创建一个页帧分配器
        调用者必须保证传递的内存映射有效，即传递的内存映射中的被标记为`可用`的页帧是真正未使用的
    */
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /*
        内存映射由BIOS/UEFI固件提供
        只能在启动过程的早期，一般情况下引导程序会提供相应的函数
        返回可用页帧的迭代器
    */
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // 从内存映射中获取可用的区域
        let regions = self.memory_map.iter();
        // filter方法跳过任何保留或其他不可用的区域
        // Bootloader为对象更新所有内存映射，因此被内核使用的页帧（代码、数据或堆栈）或存储启动信息的页帧会被标记为InUse（或类似）
        // 通过该操作确定可用页帧在其他地方没有被使用
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        // 将内存区域的迭代器转换为地址范围的迭代器
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // 将地址范围的迭代器转化为帧起始地址的迭代器，flat_map将地址范围转化为帧起始地址的迭代器，闭包函数使用`step_by`选择第4096个地址（页面大小为4KiB，即每个页帧的起始地址）
        // Bootloader会对所有可用的内存空间进行页对齐操作，因此无需额外增加对齐或舍入的代码逻辑
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // 通过起始地址创建 `PhysFrame` 类型的迭代器
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    // 从内存映射获取一个可用页帧的迭代器，然后获取索引为self.next的页帧
    // 每次调用函数进行分配时都需要重新创建usable_frame分配器，可以讨论优化实现
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/*
    非安全函数：初始化一个新的偏移页表
    调用者必须保证完整的物理内存
    函数仅能调用一次，以避免可变引用会触发未知的行为
*/
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}
