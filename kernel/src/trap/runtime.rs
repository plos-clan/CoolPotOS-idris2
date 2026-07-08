use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::idris2_runtime::{Value, idris2_newReference, idris2_removeReference};

use super::TrapFrame;

static TRAP_HANDLER: AtomicPtr<Value> = AtomicPtr::new(ptr::null_mut());
static CURRENT_TRAP_FRAME: AtomicPtr<TrapFrame> = AtomicPtr::new(ptr::null_mut());

pub(crate) fn set_handler(handler: *mut Value) {
    let retained = unsafe { idris2_newReference(handler) };
    let previous = TRAP_HANDLER.swap(retained, Ordering::AcqRel);
    if !previous.is_null() {
        unsafe { idris2_removeReference(previous) };
    }
}

pub(crate) fn handler() -> *mut Value {
    TRAP_HANDLER.load(Ordering::Acquire)
}

pub(crate) fn set_current_frame(frame: *mut TrapFrame) {
    CURRENT_TRAP_FRAME.store(frame, Ordering::Release);
}

pub(crate) fn clear_current_frame() {
    CURRENT_TRAP_FRAME.store(ptr::null_mut(), Ordering::Release);
}

fn current_frame() -> *mut TrapFrame {
    CURRENT_TRAP_FRAME.load(Ordering::Acquire)
}

pub(crate) fn with_current_frame<R>(f: impl FnOnce(&mut TrapFrame) -> R) -> Option<R> {
    let frame = current_frame();
    if frame.is_null() {
        None
    } else {
        Some(f(unsafe { &mut *frame }))
    }
}

#[inline]
pub(crate) fn halt_forever() -> ! {
    loop {
        core::hint::spin_loop();
    }
}
