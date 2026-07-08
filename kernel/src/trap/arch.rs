use core::arch::{asm, global_asm};

use super::frame::{
    A0_OFFSET, A1_OFFSET, A2_OFFSET, A3_OFFSET, A4_OFFSET, A5_OFFSET, A6_OFFSET, A7_OFFSET,
    GP_OFFSET, RA_OFFSET, S0_OFFSET, S1_OFFSET, S2_OFFSET, S3_OFFSET, S4_OFFSET, S5_OFFSET,
    S6_OFFSET, S7_OFFSET, S8_OFFSET, S9_OFFSET, S10_OFFSET, S11_OFFSET, SCAUSE_OFFSET, SEPC_OFFSET,
    SP_OFFSET, SSTATUS_OFFSET, STVAL_OFFSET, T0_OFFSET, T1_OFFSET, T2_OFFSET, T3_OFFSET, T4_OFFSET,
    T5_OFFSET, T6_OFFSET, TP_OFFSET, TRAP_FRAME_SIZE,
};

const SSTATUS_SIE: usize = 1 << 1;
const SIE_STIE: usize = 1 << 5;

unsafe extern "C" {
    fn __trap_vector();
    fn __trap_wait_for_interrupt();
    fn __trap_wait_for_interrupt_resume();
}

global_asm!(
    r#"
    .section .text.trap, "ax"
    .balign 4
    .global __trap_vector
__trap_vector:
    addi sp, sp, -{frame_size}
    sd ra, {ra}(sp)
    sd t0, {t0}(sp)
    addi t0, sp, {frame_size}
    sd t0, {sp_slot}(sp)
    sd gp, {gp}(sp)
    sd tp, {tp}(sp)
    sd t1, {t1}(sp)
    sd t2, {t2}(sp)
    sd s0, {s0}(sp)
    sd s1, {s1}(sp)
    sd a0, {a0}(sp)
    sd a1, {a1}(sp)
    sd a2, {a2}(sp)
    sd a3, {a3}(sp)
    sd a4, {a4}(sp)
    sd a5, {a5}(sp)
    sd a6, {a6}(sp)
    sd a7, {a7}(sp)
    sd s2, {s2}(sp)
    sd s3, {s3}(sp)
    sd s4, {s4}(sp)
    sd s5, {s5}(sp)
    sd s6, {s6}(sp)
    sd s7, {s7}(sp)
    sd s8, {s8}(sp)
    sd s9, {s9}(sp)
    sd s10, {s10}(sp)
    sd s11, {s11}(sp)
    sd t3, {t3}(sp)
    sd t4, {t4}(sp)
    sd t5, {t5}(sp)
    sd t6, {t6}(sp)
    csrr t0, sstatus
    sd t0, {sstatus}(sp)
    csrr t0, sepc
    sd t0, {sepc}(sp)
    csrr t0, scause
    sd t0, {scause}(sp)
    csrr t0, stval
    sd t0, {stval}(sp)
    mv a0, sp
    call kernel_trap_dispatch
    ld t0, {sepc}(sp)
    csrw sepc, t0
    ld t0, {sstatus}(sp)
    csrw sstatus, t0
    ld ra, {ra}(sp)
    ld gp, {gp}(sp)
    ld tp, {tp}(sp)
    ld t0, {t0}(sp)
    ld t1, {t1}(sp)
    ld t2, {t2}(sp)
    ld s0, {s0}(sp)
    ld s1, {s1}(sp)
    ld a0, {a0}(sp)
    ld a1, {a1}(sp)
    ld a2, {a2}(sp)
    ld a3, {a3}(sp)
    ld a4, {a4}(sp)
    ld a5, {a5}(sp)
    ld a6, {a6}(sp)
    ld a7, {a7}(sp)
    ld s2, {s2}(sp)
    ld s3, {s3}(sp)
    ld s4, {s4}(sp)
    ld s5, {s5}(sp)
    ld s6, {s6}(sp)
    ld s7, {s7}(sp)
    ld s8, {s8}(sp)
    ld s9, {s9}(sp)
    ld s10, {s10}(sp)
    ld s11, {s11}(sp)
    ld t3, {t3}(sp)
    ld t4, {t4}(sp)
    ld t5, {t5}(sp)
    ld t6, {t6}(sp)
    addi sp, sp, {frame_size}
    sret

    .balign 4
    .global __trap_wait_for_interrupt
    .global __trap_wait_for_interrupt_resume
__trap_wait_for_interrupt:
    wfi
__trap_wait_for_interrupt_resume:
    ret
"#,
    frame_size = const TRAP_FRAME_SIZE,
    ra = const RA_OFFSET,
    sp_slot = const SP_OFFSET,
    gp = const GP_OFFSET,
    tp = const TP_OFFSET,
    t0 = const T0_OFFSET,
    t1 = const T1_OFFSET,
    t2 = const T2_OFFSET,
    s0 = const S0_OFFSET,
    s1 = const S1_OFFSET,
    a0 = const A0_OFFSET,
    a1 = const A1_OFFSET,
    a2 = const A2_OFFSET,
    a3 = const A3_OFFSET,
    a4 = const A4_OFFSET,
    a5 = const A5_OFFSET,
    a6 = const A6_OFFSET,
    a7 = const A7_OFFSET,
    s2 = const S2_OFFSET,
    s3 = const S3_OFFSET,
    s4 = const S4_OFFSET,
    s5 = const S5_OFFSET,
    s6 = const S6_OFFSET,
    s7 = const S7_OFFSET,
    s8 = const S8_OFFSET,
    s9 = const S9_OFFSET,
    s10 = const S10_OFFSET,
    s11 = const S11_OFFSET,
    t3 = const T3_OFFSET,
    t4 = const T4_OFFSET,
    t5 = const T5_OFFSET,
    t6 = const T6_OFFSET,
    sstatus = const SSTATUS_OFFSET,
    sepc = const SEPC_OFFSET,
    scause = const SCAUSE_OFFSET,
    stval = const STVAL_OFFSET,
);

pub(crate) fn install() {
    unsafe {
        write_stvec(__trap_vector as *const () as usize);
    }
}

pub(crate) fn enable_supervisor_interrupts() {
    unsafe {
        asm!("csrs sstatus, {}", in(reg) SSTATUS_SIE, options(nostack));
    }
}

pub(crate) fn disable_supervisor_interrupts() {
    unsafe {
        asm!("csrc sstatus, {}", in(reg) SSTATUS_SIE, options(nostack));
    }
}

pub(crate) fn enable_timer_interrupts() {
    unsafe {
        asm!("csrs sie, {}", in(reg) SIE_STIE, options(nostack));
    }
}

pub(crate) fn schedule_timer(delta: u64) {
    let deadline = read_time().wrapping_add(delta);
    write_stimecmp(deadline);
}

pub(crate) fn wait_for_interrupt() {
    unsafe { __trap_wait_for_interrupt() }
}

pub(crate) fn trigger_breakpoint() {
    unsafe {
        asm!("ebreak", options(nomem, nostack));
    }
}

pub(crate) fn breakpoint_next_pc(sepc: u64) -> u64 {
    let instruction = unsafe { (sepc as *const u16).read_unaligned() };
    let length = if (instruction & 0b11) == 0b11 { 4 } else { 2 };

    sepc.wrapping_add(length)
}

pub(crate) fn is_wait_for_interrupt_pc(sepc: u64) -> bool {
    sepc == __trap_wait_for_interrupt as *const () as usize as u64
}

pub(crate) fn wait_for_interrupt_resume_pc() -> u64 {
    __trap_wait_for_interrupt_resume as *const () as usize as u64
}

#[inline]
fn read_time() -> u64 {
    let value: u64;
    unsafe {
        asm!("csrr {}, time", out(reg) value, options(nomem, nostack));
    }
    value
}

#[inline]
unsafe fn write_stvec(address: usize) {
    unsafe {
        asm!("csrw stvec, {}", in(reg) (address & !0b11), options(nostack));
    }
}

#[inline]
fn write_stimecmp(deadline: u64) {
    unsafe {
        asm!("csrw stimecmp, {}", in(reg) deadline, options(nostack));
    }
}
