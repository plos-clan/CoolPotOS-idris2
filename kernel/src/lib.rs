#![no_std]

mod allocator;
mod idris2_runtime;
pub mod serial;

use core::hint::spin_loop;
use core::panic::PanicInfo;

use idris2_runtime::{Value, idris2_removeReference, idris2_trampoline};

unsafe extern "C" {
    fn __mainExpression_0() -> *mut Value;
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_entry() -> ! {
    let result = unsafe { idris2_trampoline(__mainExpression_0()) };
    unsafe { idris2_removeReference(result) };

    loop {
        spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo<'_>) -> ! {
    loop {
        spin_loop();
    }
}
