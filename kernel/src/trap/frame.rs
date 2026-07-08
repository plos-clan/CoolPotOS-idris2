use core::mem::offset_of;

#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct TrapFrame {
    pub(crate) ra: u64,
    pub(crate) sp: u64,
    pub(crate) gp: u64,
    pub(crate) tp: u64,
    pub(crate) t0: u64,
    pub(crate) t1: u64,
    pub(crate) t2: u64,
    pub(crate) s0: u64,
    pub(crate) s1: u64,
    pub(crate) a0: u64,
    pub(crate) a1: u64,
    pub(crate) a2: u64,
    pub(crate) a3: u64,
    pub(crate) a4: u64,
    pub(crate) a5: u64,
    pub(crate) a6: u64,
    pub(crate) a7: u64,
    pub(crate) s2: u64,
    pub(crate) s3: u64,
    pub(crate) s4: u64,
    pub(crate) s5: u64,
    pub(crate) s6: u64,
    pub(crate) s7: u64,
    pub(crate) s8: u64,
    pub(crate) s9: u64,
    pub(crate) s10: u64,
    pub(crate) s11: u64,
    pub(crate) t3: u64,
    pub(crate) t4: u64,
    pub(crate) t5: u64,
    pub(crate) t6: u64,
    pub(crate) sstatus: u64,
    pub(crate) sepc: u64,
    pub(crate) scause: u64,
    pub(crate) stval: u64,
    pub(crate) _reserved: u64,
}

pub(crate) const TRAP_FRAME_SIZE: usize = core::mem::size_of::<TrapFrame>();

pub(crate) const RA_OFFSET: usize = offset_of!(TrapFrame, ra);
pub(crate) const SP_OFFSET: usize = offset_of!(TrapFrame, sp);
pub(crate) const GP_OFFSET: usize = offset_of!(TrapFrame, gp);
pub(crate) const TP_OFFSET: usize = offset_of!(TrapFrame, tp);
pub(crate) const T0_OFFSET: usize = offset_of!(TrapFrame, t0);
pub(crate) const T1_OFFSET: usize = offset_of!(TrapFrame, t1);
pub(crate) const T2_OFFSET: usize = offset_of!(TrapFrame, t2);
pub(crate) const S0_OFFSET: usize = offset_of!(TrapFrame, s0);
pub(crate) const S1_OFFSET: usize = offset_of!(TrapFrame, s1);
pub(crate) const A0_OFFSET: usize = offset_of!(TrapFrame, a0);
pub(crate) const A1_OFFSET: usize = offset_of!(TrapFrame, a1);
pub(crate) const A2_OFFSET: usize = offset_of!(TrapFrame, a2);
pub(crate) const A3_OFFSET: usize = offset_of!(TrapFrame, a3);
pub(crate) const A4_OFFSET: usize = offset_of!(TrapFrame, a4);
pub(crate) const A5_OFFSET: usize = offset_of!(TrapFrame, a5);
pub(crate) const A6_OFFSET: usize = offset_of!(TrapFrame, a6);
pub(crate) const A7_OFFSET: usize = offset_of!(TrapFrame, a7);
pub(crate) const S2_OFFSET: usize = offset_of!(TrapFrame, s2);
pub(crate) const S3_OFFSET: usize = offset_of!(TrapFrame, s3);
pub(crate) const S4_OFFSET: usize = offset_of!(TrapFrame, s4);
pub(crate) const S5_OFFSET: usize = offset_of!(TrapFrame, s5);
pub(crate) const S6_OFFSET: usize = offset_of!(TrapFrame, s6);
pub(crate) const S7_OFFSET: usize = offset_of!(TrapFrame, s7);
pub(crate) const S8_OFFSET: usize = offset_of!(TrapFrame, s8);
pub(crate) const S9_OFFSET: usize = offset_of!(TrapFrame, s9);
pub(crate) const S10_OFFSET: usize = offset_of!(TrapFrame, s10);
pub(crate) const S11_OFFSET: usize = offset_of!(TrapFrame, s11);
pub(crate) const T3_OFFSET: usize = offset_of!(TrapFrame, t3);
pub(crate) const T4_OFFSET: usize = offset_of!(TrapFrame, t4);
pub(crate) const T5_OFFSET: usize = offset_of!(TrapFrame, t5);
pub(crate) const T6_OFFSET: usize = offset_of!(TrapFrame, t6);
pub(crate) const SSTATUS_OFFSET: usize = offset_of!(TrapFrame, sstatus);
pub(crate) const SEPC_OFFSET: usize = offset_of!(TrapFrame, sepc);
pub(crate) const SCAUSE_OFFSET: usize = offset_of!(TrapFrame, scause);
pub(crate) const STVAL_OFFSET: usize = offset_of!(TrapFrame, stval);
