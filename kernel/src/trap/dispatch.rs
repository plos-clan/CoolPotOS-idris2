use core::ptr;

use crate::idris2_runtime::{
    idris2_apply_closure, idris2_newReference, idris2_removeReference, idris2_trampoline,
};
use crate::serial;

use super::{TrapFrame, runtime};

#[unsafe(no_mangle)]
extern "C" fn kernel_trap_dispatch(frame: *mut TrapFrame) {
    let handler = runtime::handler();
    if handler.is_null() {
        serial::write_str("trap: synchronous Idris handler is not registered\r\n");
        runtime::halt_forever();
    }

    runtime::set_current_frame(frame);
    let applied = unsafe { idris2_apply_closure(idris2_newReference(handler), ptr::null_mut()) };
    let result = unsafe { idris2_trampoline(applied) };
    runtime::clear_current_frame();
    unsafe { idris2_removeReference(result) };
}
