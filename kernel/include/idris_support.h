#pragma once

#include <stdint.h>

int idris2_isNull(void *ptr);
void *idris2_getNull(void);
char *idris2_getString(void *ptr);
int idris2_getErrno(void);
char *idris2_strerror(int errnum);

char *idris2_getStr(void);
void idris2_putStr(char *text);

void idris2_sleep(int sec);
void idris2_usleep(int usec);
int idris2_time(void);

int idris2_getArgCount(void);
void idris2_setArgs(int argc, char *argv[]);
char *idris2_getArg(int index);
char *idris2_getEnvPair(int index);

int idris2_getPID(void);
long idris2_getNProcessors(void);

void kernel_trap_install(void);
void kernel_trap_enable_timer_interrupts(void);
void kernel_trap_enable_supervisor_interrupts(void);
void kernel_trap_disable_supervisor_interrupts(void);
void kernel_trap_schedule_timer(uint64_t delta);
uint64_t kernel_trap_breakpoint_next_pc(uint64_t sepc);
int kernel_trap_is_wait_for_interrupt_pc(uint64_t sepc);
uint64_t kernel_trap_wait_for_interrupt_resume_pc(void);
void kernel_trap_wait_for_interrupt(void);
void kernel_trap_trigger_breakpoint(void);
void kernel_trap_set_sync_handler_closure(Value *handler);
uint64_t kernel_trap_current_ra(void);
uint64_t kernel_trap_current_sp(void);
uint64_t kernel_trap_current_sstatus(void);
uint64_t kernel_trap_current_sepc(void);
uint64_t kernel_trap_current_scause(void);
uint64_t kernel_trap_current_stval(void);
void kernel_trap_set_sepc(uint64_t sepc);
void kernel_debug_write_hex_u64(uint64_t value);
void kernel_trap_halt_forever(void);
