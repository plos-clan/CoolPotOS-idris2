module Trap.Prim

import Data.Bits
import PrimIO

%foreign "RefC:kernel_trap_install,kernel,idris_support.h"
prim__trapInstall : PrimIO ()

%foreign "RefC:kernel_trap_enable_timer_interrupts,kernel,idris_support.h"
prim__trapEnableTimerInterrupts : PrimIO ()

%foreign "RefC:kernel_trap_enable_supervisor_interrupts,kernel,idris_support.h"
prim__trapEnableSupervisorInterrupts : PrimIO ()

%foreign "RefC:kernel_trap_disable_supervisor_interrupts,kernel,idris_support.h"
prim__trapDisableSupervisorInterrupts : PrimIO ()

%foreign "RefC:kernel_trap_schedule_timer,kernel,idris_support.h"
prim__trapScheduleTimer : Bits64 -> PrimIO ()

%foreign "RefC:kernel_trap_breakpoint_next_pc,kernel,idris_support.h"
prim__trapBreakpointNextPC : Bits64 -> PrimIO Bits64

%foreign "RefC:kernel_trap_is_wait_for_interrupt_pc,kernel,idris_support.h"
prim__trapIsWaitForInterruptPC : Bits64 -> PrimIO Int

%foreign "RefC:kernel_trap_wait_for_interrupt_resume_pc,kernel,idris_support.h"
prim__trapWaitForInterruptResumePC : PrimIO Bits64

%foreign "RefC:kernel_trap_wait_for_interrupt,kernel,idris_support.h"
prim__trapWaitForInterrupt : PrimIO ()

%foreign "RefC:kernel_trap_trigger_breakpoint,kernel,idris_support.h"
prim__trapTriggerBreakpoint : PrimIO ()

%foreign "RefC:kernel_trap_set_sync_handler_closure,kernel,idris_support.h"
prim__trapSetSyncHandlerClosure : IO () -> PrimIO ()

%foreign "RefC:kernel_trap_current_ra,kernel,idris_support.h"
prim__trapCurrentRA : PrimIO Bits64

%foreign "RefC:kernel_trap_current_sp,kernel,idris_support.h"
prim__trapCurrentSP : PrimIO Bits64

%foreign "RefC:kernel_trap_current_sstatus,kernel,idris_support.h"
prim__trapCurrentSStatus : PrimIO Bits64

%foreign "RefC:kernel_trap_current_sepc,kernel,idris_support.h"
prim__trapCurrentSEPC : PrimIO Bits64

%foreign "RefC:kernel_trap_current_scause,kernel,idris_support.h"
prim__trapCurrentSCause : PrimIO Bits64

%foreign "RefC:kernel_trap_current_stval,kernel,idris_support.h"
prim__trapCurrentSTVal : PrimIO Bits64

%foreign "RefC:kernel_trap_set_sepc,kernel,idris_support.h"
prim__trapSetSEPC : Bits64 -> PrimIO ()

%foreign "RefC:kernel_debug_write_hex_u64,kernel,idris_support.h"
prim__trapWriteHex : Bits64 -> PrimIO ()

%foreign "RefC:kernel_trap_halt_forever,kernel,idris_support.h"
prim__trapHaltForever : PrimIO ()

public export
trapInstall : IO ()
trapInstall = primIO prim__trapInstall

export
trapEnableTimerInterrupts : IO ()
trapEnableTimerInterrupts = primIO prim__trapEnableTimerInterrupts

export
trapEnableSupervisorInterrupts : IO ()
trapEnableSupervisorInterrupts = primIO prim__trapEnableSupervisorInterrupts

export
trapDisableSupervisorInterrupts : IO ()
trapDisableSupervisorInterrupts = primIO prim__trapDisableSupervisorInterrupts

export
trapScheduleTimer : Bits64 -> IO ()
trapScheduleTimer delta = primIO (prim__trapScheduleTimer delta)

export
trapBreakpointNextPC : Bits64 -> IO Bits64
trapBreakpointNextPC pc = primIO (prim__trapBreakpointNextPC pc)

export
trapIsWaitForInterruptPC : Bits64 -> IO Int
trapIsWaitForInterruptPC pc = primIO (prim__trapIsWaitForInterruptPC pc)

export
trapWaitForInterruptResumePC : IO Bits64
trapWaitForInterruptResumePC = primIO prim__trapWaitForInterruptResumePC

export
trapWaitForInterrupt : IO ()
trapWaitForInterrupt = primIO prim__trapWaitForInterrupt

export
trapTriggerBreakpoint : IO ()
trapTriggerBreakpoint = primIO prim__trapTriggerBreakpoint

export
trapSetSyncHandlerClosure : IO () -> IO ()
trapSetSyncHandlerClosure handler = primIO (prim__trapSetSyncHandlerClosure handler)

export
trapCurrentRA : IO Bits64
trapCurrentRA = primIO prim__trapCurrentRA

export
trapCurrentSP : IO Bits64
trapCurrentSP = primIO prim__trapCurrentSP

export
trapCurrentSStatus : IO Bits64
trapCurrentSStatus = primIO prim__trapCurrentSStatus

export
trapCurrentSEPC : IO Bits64
trapCurrentSEPC = primIO prim__trapCurrentSEPC

export
trapCurrentSCause : IO Bits64
trapCurrentSCause = primIO prim__trapCurrentSCause

export
trapCurrentSTVal : IO Bits64
trapCurrentSTVal = primIO prim__trapCurrentSTVal

export
trapSetSEPC : Bits64 -> IO ()
trapSetSEPC sepc = primIO (prim__trapSetSEPC sepc)

export
trapWriteHex : Bits64 -> IO ()
trapWriteHex value = primIO (prim__trapWriteHex value)

export
trapHaltForever : IO ()
trapHaltForever = primIO prim__trapHaltForever
