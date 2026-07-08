module Trap.Handler

import Trap.Core
import Trap.Prim

trapLogField : String -> Bits64 -> IO ()
trapLogField label value = do
    putStr label
    trapWriteHex value
    putStrLn ""

logInterrupt : Bits64 -> IO ()
logInterrupt 0 = putStrLn "trap: user software interrupt"
logInterrupt 1 = putStrLn "trap: supervisor software interrupt"
logInterrupt 4 = putStrLn "trap: user timer interrupt"
logInterrupt 5 = putStrLn "trap: supervisor timer interrupt"
logInterrupt 8 = putStrLn "trap: user external interrupt"
logInterrupt 9 = putStrLn "trap: supervisor external interrupt"
logInterrupt code = trapLogField "trap: unknown interrupt code=" code

logException : Bits64 -> IO ()
logException 0 = putStrLn "trap: instruction address misaligned"
logException 1 = putStrLn "trap: instruction access fault"
logException 2 = putStrLn "trap: illegal instruction"
logException 3 = putStrLn "trap: breakpoint exception"
logException 4 = putStrLn "trap: load address misaligned"
logException 5 = putStrLn "trap: load access fault"
logException 6 = putStrLn "trap: store address misaligned"
logException 7 = putStrLn "trap: store access fault"
logException 8 = putStrLn "trap: user environment call"
logException 9 = putStrLn "trap: supervisor environment call"
logException 12 = putStrLn "trap: instruction page fault"
logException 13 = putStrLn "trap: load page fault"
logException 15 = putStrLn "trap: store page fault"
logException code = trapLogField "trap: unknown exception code=" code

logTrap : Bits64 -> IO ()
logTrap scause =
    if isInterrupt scause
        then logInterrupt (causeCode scause)
        else logException (causeCode scause)

dumpTrapContext : TrapSnapshot -> IO ()
dumpTrapContext snapshot = do
    trapLogField "trap: scause=" snapshot.scause
    trapLogField "trap: sepc=" snapshot.sepc
    trapLogField "trap: stval=" snapshot.stval
    trapLogField "trap: sstatus=" snapshot.sstatus
    trapLogField "trap: ra=" snapshot.ra
    trapLogField "trap: sp=" snapshot.sp

readCurrentTrap : IO TrapSnapshot
readCurrentTrap = do
    ra <- trapCurrentRA
    sp <- trapCurrentSP
    sstatus <- trapCurrentSStatus
    sepc <- trapCurrentSEPC
    scause <- trapCurrentSCause
    stval <- trapCurrentSTVal
    pure (MkTrapSnapshot ra sp sstatus sepc scause stval)

resumePC : TrapSnapshot -> IO Bits64
resumePC snapshot =
    if isSupervisorTimer snapshot.scause
        then do
            inWait <- trapIsWaitForInterruptPC snapshot.sepc
            if inWait /= 0
                then trapWaitForInterruptResumePC
                else pure snapshot.sepc
        else if isBreakpoint snapshot.scause
            then trapBreakpointNextPC snapshot.sepc
            else if isEnvCall snapshot.scause
                then pure (snapshot.sepc + ecallInstructionLength)
                else do
                    putStrLn "trap: trap is fatal in current Idris stage"
                    trapHaltForever
                    pure snapshot.sepc

public export
handleCurrentTrap : IO ()
handleCurrentTrap = do
    snapshot <- readCurrentTrap
    logTrap snapshot.scause
    if shouldDumpContext snapshot.scause
        then dumpTrapContext snapshot
        else pure ()

    if isSupervisorTimer snapshot.scause
        then trapScheduleTimer timerIntervalTicks
        else pure ()

    next <- resumePC snapshot
    trapSetSEPC next

export
trapSetSyncHandler : IO ()
trapSetSyncHandler = trapSetSyncHandlerClosure handleCurrentTrap

