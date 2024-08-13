use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};

pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /*
        非安全函数
        从传递的内存map中创建一个FrameAllocator，调用者必须保证传递的内存map是有效的
        即在其中被标记为“可用”的页帧都是真正未使用的
    */
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /*
        内存图由BIOS/UEFI固件提供
        只能在启动过程的早期被查询，一般情况下引导程序会提供相应的函数
        函数返回内存映射中指定的可用页帧的迭代器类型
    */
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // 从内存 map 中获取可用的区域，内存映射转换为多个MemoryRegion的迭代器
        let regions = self.memory_map.iter();
        // filter方法跳过任何保留或其他不可用的区域
        // Bootloader为其创建的所有映射更新了内存地图，所以被内核使用的页帧（代码、数据或堆栈）或存储启动信息的页帧会被标记为InUse或类似，通过如此操作确定可用页帧在其他地方没有被使用
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        // 将每个区域映射到其地址范围，map组合器和range语法将我们的内存区域迭代器转化为地址范围的迭代器
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // 转化为一个帧起始地址的迭代器，flat_map将地址范围转化为帧起始地址的迭代器，闭包函数使用`step_by`选择每4096个地址（页面大小为4KiB，即每个页帧的起始地址）
        // 使用flat_map的原因：Bootloader会对所有可用的内存区域进行页对齐
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // 从起始地址创建`PhysFrame`类型
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        // 从内存映射获取一个可用页帧的迭代器，然后获取索引为self.next的页帧
        // 每次调用函数进行分配时都需要重新创建usable_frame分配器，可以讨论优化实现
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

/* 不安全函数
   调用者必须保证完整的物理内存
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

// 示例：为给定的页面创建一个实例映射到页帧`0xb8000`
pub fn create_example_mapping(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        // 非安全测试用例
        mapper.map_to(page, frame, flags, frame_allocator)
    };

    map_to_result.expect("map_to failed").flush();
}

// Fake FrameAllocator return None
pub struct EmptyFrameAllocator;

// 实现FrameAllocator属于非安全的，实现者必须保证分配器只产生未使用的页帧
unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        None
    }
}
