use super::{align_up, Locked};
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr,
};
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    // Initialize the bump allocator with the given heap bounds.
    // 函数非安全，调用者必须保证给定内存范围未使用且仅能调用一次
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        // heap_start 和 heap_end 持续追踪内存区域的上下界，保证地址有效，否则allocator会返回无效内存
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        // next 总是指向堆中的未使用字节
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };
        if alloc_end > bump.heap_end {
            ptr::null_mut()
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();

        bump.allocations -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}
