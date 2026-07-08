module Trap

import Trap.Prim
import Trap.Handler
import Trap.Runtime

public export
trapInstall : IO ()
trapInstall = Trap.Prim.trapInstall

public export
handleCurrentTrap : IO ()
handleCurrentTrap = Trap.Handler.handleCurrentTrap

public export
partial
runKernel : IO ()
runKernel = Trap.Runtime.runKernel
