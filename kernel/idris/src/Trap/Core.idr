module Trap.Core

import Data.Bits

interruptBit : Bits64
interruptBit = 0x8000000000000000

causeMask : Bits64
causeMask = 0x7fffffffffffffff

supervisorTimerInterrupt : Bits64
supervisorTimerInterrupt = 5

breakpointException : Bits64
breakpointException = 3

userEnvCallException : Bits64
userEnvCallException = 8

supervisorEnvCallException : Bits64
supervisorEnvCallException = 9

export
ecallInstructionLength : Bits64
ecallInstructionLength = 4

export
timerIntervalTicks : Bits64
timerIntervalTicks = 50000000

public export
record TrapSnapshot where
    constructor MkTrapSnapshot
    ra : Bits64
    sp : Bits64
    sstatus : Bits64
    sepc : Bits64
    scause : Bits64
    stval : Bits64

export
isInterrupt : Bits64 -> Bool
isInterrupt scause = (scause .&. interruptBit) /= 0

export
causeCode : Bits64 -> Bits64
causeCode scause = scause .&. causeMask

export
isSupervisorTimer : Bits64 -> Bool
isSupervisorTimer scause =
    isInterrupt scause && causeCode scause == supervisorTimerInterrupt

export
isBreakpoint : Bits64 -> Bool
isBreakpoint scause =
    not (isInterrupt scause) && causeCode scause == breakpointException

export
isEnvCall : Bits64 -> Bool
isEnvCall scause =
    let code = causeCode scause in
        not (isInterrupt scause) &&
            (code == userEnvCallException || code == supervisorEnvCallException)

export
shouldDumpContext : Bits64 -> Bool
shouldDumpContext scause = not (isSupervisorTimer scause)
