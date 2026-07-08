use core::alloc::{GlobalAlloc, Layout};
use core::mem::MaybeUninit;
use core::ptr;

use spin::Mutex as SpinMutex;
use talc::{DefaultBinning, TalcLock, min_first_heap_size, source::Claim};

const HEAP_SIZE: usize = min_first_heap_size::<DefaultBinning>() + (1024 * 1024);

#[global_allocator]
static ALLOCATOR: TalcLock<SpinMutex<()>, Claim> = TalcLock::new(unsafe {
    static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    Claim::array(&raw mut HEAP)
});

pub unsafe fn alloc_zeroed(size: usize, align: usize) -> *mut u8 {
    if size == 0 {
        return ptr::null_mut();
    }

    unsafe { GlobalAlloc::alloc_zeroed(&ALLOCATOR, layout(size, align)) }
}

pub unsafe fn dealloc(ptr: *mut u8, size: usize, align: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }

    unsafe { GlobalAlloc::dealloc(&ALLOCATOR, ptr, layout(size, align)) }
}

fn layout(size: usize, align: usize) -> Layout {
    match Layout::from_size_align(size, align) {
        Ok(layout) => layout,
        Err(_) => panic!("invalid talc allocation layout"),
    }
}
