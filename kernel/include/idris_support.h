#pragma once

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
