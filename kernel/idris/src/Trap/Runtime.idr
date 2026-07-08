module Trap.Runtime

import Trap.Core
import Trap.Handler
import Trap.Prim

trapInit : IO ()
trapInit = do
    putStrLn "trap: installing supervisor trap vector"
    trapInstall
    trapSetSyncHandler

armTimerInterrupts : IO ()
armTimerInterrupts = do
    trapEnableTimerInterrupts
    trapScheduleTimer timerIntervalTicks
    putStrLn "trap: timer interrupt armed"

runBreakpointSelfTest : IO ()
runBreakpointSelfTest = do
    putStrLn "trap: running post-init breakpoint self-test"
    trapTriggerBreakpoint

partial
trapLoop : IO ()
trapLoop = do
    trapEnableSupervisorInterrupts
    trapWaitForInterrupt
    trapDisableSupervisorInterrupts
    trapLoop

public export
partial
runKernel : IO ()
runKernel = do
    putStrLn "idris: serial output online"
    trapInit
    runBreakpointSelfTest
    armTimerInterrupts
    trapLoop
