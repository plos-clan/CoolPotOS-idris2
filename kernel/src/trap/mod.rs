mod arch;
mod dispatch;
mod frame;
mod runtime;

use crate::serial;
use crate::idris2_runtime::Value;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_install() {
    arch::install();
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_enable_timer_interrupts() {
    arch::enable_timer_interrupts();
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_enable_supervisor_interrupts() {
    arch::enable_supervisor_interrupts();
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_disable_supervisor_interrupts() {
    arch::disable_supervisor_interrupts();
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_schedule_timer(delta: u64) {
    arch::schedule_timer(delta);
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_breakpoint_next_pc(sepc: u64) -> u64 {
    arch::breakpoint_next_pc(sepc)
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_is_wait_for_interrupt_pc(sepc: u64) -> i32 {
    i32::from(arch::is_wait_for_interrupt_pc(sepc))
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_wait_for_interrupt_resume_pc() -> u64 {
    arch::wait_for_interrupt_resume_pc()
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_wait_for_interrupt() {
    arch::wait_for_interrupt();
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_trigger_breakpoint() {
    arch::trigger_breakpoint();
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_set_sync_handler_closure(handler: *mut Value) {
    runtime::set_handler(handler);
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_current_ra() -> u64 {
    runtime::with_current_frame(|frame| frame.ra).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_current_sp() -> u64 {
    runtime::with_current_frame(|frame| frame.sp).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_current_sstatus() -> u64 {
    runtime::with_current_frame(|frame| frame.sstatus).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_current_sepc() -> u64 {
    runtime::with_current_frame(|frame| frame.sepc).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_current_scause() -> u64 {
    runtime::with_current_frame(|frame| frame.scause).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_current_stval() -> u64 {
    runtime::with_current_frame(|frame| frame.stval).unwrap_or(0)
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_set_sepc(sepc: u64) {
    if runtime::with_current_frame(|frame| {
        frame.sepc = sepc;
    })
    .is_none()
    {
        serial::write_str("trap: no active frame when setting sepc\r\n");
        runtime::halt_forever();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_debug_write_hex_u64(value: u64) {
    serial::write_hex_u64(value);
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_trap_halt_forever() -> ! {
    runtime::halt_forever()
}

pub(crate) use frame::TrapFrame;
