#pragma once

#include <stddef.h>
#include <stdint.h>

#define NO_TAG 0
#define BITS64_TAG 4
#define INT64_TAG 8
#define INTEGER_TAG 9
#define DOUBLE_TAG 10
#define STRING_TAG 12
#define CLOSURE_TAG 15
#define CONSTRUCTOR_TAG 17
#define IOREF_TAG 20
#define ARRAY_TAG 21
#define POINTER_TAG 22

#define IDRIS2_VP_REFCOUNTER_MAX UINT16_MAX
#define IDRIS2_VP_INT_SHIFT 32
#define IDRIS2_STOCKVAL(t) { IDRIS2_VP_REFCOUNTER_MAX, t, 0 }

#define idris2_isUnique(x) ((x)->header.refCounter == 1)
#define idris2_vp_is_unboxed(p) (((uintptr_t)(p) & 1u) != 0)

typedef struct {
  uint16_t refCounter;
  uint8_t tag;
  uint8_t reserved;
} Value_header;

typedef struct {
  Value_header header;
} Value;

typedef struct {
  Value_header header;
  uint64_t ui64;
} Value_Bits64;

typedef struct {
  Value_header header;
  int64_t i64;
} Value_Int64;

typedef struct {
  Value_header header;
  int64_t i;
} Value_Integer;

typedef struct {
  Value_header header;
  double d;
} Value_Double;

typedef struct {
  Value_header header;
  char *str;
} Value_String;

typedef struct {
  Value_header header;
  int32_t total;
  int32_t tag;
  char const *name;
  Value *args[];
} Value_Constructor;

typedef struct {
  Value_header header;
  void *f;
  uint8_t arity;
  uint8_t filled;
  Value *args[];
} Value_Closure;

typedef struct {
  Value_header header;
  Value *v;
} Value_IORef;

typedef struct {
  Value_header header;
  void *p;
} Value_Pointer;

typedef struct {
  Value_header header;
  int32_t capacity;
  Value **arr;
} Value_Array;

void idris2_missing_ffi(void);
Value *idris2_newValue(size_t size);
Value *idris2_newReference(Value *source);
void idris2_removeReference(Value *source);
void idris2_removeReuseConstructor(Value_Constructor *constr);

Value_Constructor *idris2_newConstructor(int total, int tag);
Value_Closure *idris2_mkClosure(Value *(*f)(), uint8_t arity, uint8_t filled);

Value *idris2_apply_closure(Value *closure, Value *arg);
Value *idris2_tailcall_apply_closure(Value *closure, Value *arg);
Value *idris2_trampoline(Value *value);

int64_t idris2_extractInt(Value *value);
void idris2_dumpMemoryStats(void);

Value *idris2_mkBits64(uint64_t value);
Value *idris2_mkInt64(int64_t value);
Value *idris2_mkDouble(double value);
Value *idris2_mkIntegerLiteral(char const *text);
Value *idris2_getPredefinedInteger(int64_t value);

Value *idris2_add_Integer(Value *lhs, Value *rhs);
Value *idris2_sub_Integer(Value *lhs, Value *rhs);
Value *idris2_cast_Integer_to_Int64(Value *value);

extern Value_Int64 const idris2_predefined_Int64[100];
extern Value_Bits64 const idris2_predefined_Bits64[100];
extern Value_Integer const idris2_predefined_Integer[100];

static inline uint8_t idris2_vp_to_Bits8(Value *value) {
  return (uint8_t)(((uintptr_t)value) >> IDRIS2_VP_INT_SHIFT);
}

static inline uint16_t idris2_vp_to_Bits16(Value *value) {
  return (uint16_t)(((uintptr_t)value) >> IDRIS2_VP_INT_SHIFT);
}

static inline uint32_t idris2_vp_to_Bits32(Value *value) {
  return (uint32_t)(((uintptr_t)value) >> IDRIS2_VP_INT_SHIFT);
}

static inline int8_t idris2_vp_to_Int8(Value *value) {
  return (int8_t)(((uintptr_t)value) >> IDRIS2_VP_INT_SHIFT);
}

static inline int16_t idris2_vp_to_Int16(Value *value) {
  return (int16_t)(((uintptr_t)value) >> IDRIS2_VP_INT_SHIFT);
}

static inline int32_t idris2_vp_to_Int32(Value *value) {
  return (int32_t)(((uintptr_t)value) >> IDRIS2_VP_INT_SHIFT);
}

static inline uint8_t idris2_vp_to_Char(Value *value) {
  return idris2_vp_to_Bits8(value);
}

static inline uint64_t idris2_vp_to_Bits64(Value *value) {
  return ((Value_Bits64 *)value)->ui64;
}

static inline int64_t idris2_vp_to_Int64(Value *value) {
  return ((Value_Int64 *)value)->i64;
}

static inline double idris2_vp_to_Double(Value *value) {
  return ((Value_Double *)value)->d;
}

static inline Value *idris2_mkBits8(uint8_t value) {
  return (Value *)((((uintptr_t)value) << IDRIS2_VP_INT_SHIFT) | 1u);
}

static inline Value *idris2_mkBits16(uint16_t value) {
  return (Value *)((((uintptr_t)value) << IDRIS2_VP_INT_SHIFT) | 1u);
}

static inline Value *idris2_mkBits32(uint32_t value) {
  return (Value *)((((uintptr_t)value) << IDRIS2_VP_INT_SHIFT) | 1u);
}

static inline Value *idris2_mkInt8(int8_t value) {
  return (Value *)((((uintptr_t)(uint32_t)(int32_t)value) << IDRIS2_VP_INT_SHIFT) | 1u);
}

static inline Value *idris2_mkInt16(int16_t value) {
  return (Value *)((((uintptr_t)(uint32_t)(int32_t)value) << IDRIS2_VP_INT_SHIFT) | 1u);
}

static inline Value *idris2_mkInt32(int32_t value) {
  return (Value *)((((uintptr_t)(uint32_t)value) << IDRIS2_VP_INT_SHIFT) | 1u);
}

static inline Value *idris2_mkBool(uint8_t value) {
  return idris2_mkInt8((int8_t)value);
}

static inline Value *idris2_mkChar(uint8_t value) {
  return idris2_mkBits8(value);
}

static inline Value *idris2_add_Int64(Value *lhs, Value *rhs) {
  return idris2_mkInt64(idris2_vp_to_Int64(lhs) + idris2_vp_to_Int64(rhs));
}

static inline Value *idris2_add_Bits64(Value *lhs, Value *rhs) {
  return idris2_mkBits64(idris2_vp_to_Bits64(lhs) + idris2_vp_to_Bits64(rhs));
}

static inline Value *idris2_and_Bits64(Value *lhs, Value *rhs) {
  return idris2_mkBits64(idris2_vp_to_Bits64(lhs) & idris2_vp_to_Bits64(rhs));
}

static inline Value *idris2_eq_Int64(Value *lhs, Value *rhs) {
  return idris2_mkBool(idris2_vp_to_Int64(lhs) == idris2_vp_to_Int64(rhs));
}

static inline Value *idris2_eq_Bits64(Value *lhs, Value *rhs) {
  return idris2_mkBool(idris2_vp_to_Bits64(lhs) == idris2_vp_to_Bits64(rhs));
}

static inline Value *idris2_eq_Integer(Value *lhs, Value *rhs) {
  return idris2_mkBool(((Value_Integer *)lhs)->i == ((Value_Integer *)rhs)->i);
}

static inline Value *idris2_lt_Integer(Value *lhs, Value *rhs) {
  return idris2_mkBool(((Value_Integer *)lhs)->i < ((Value_Integer *)rhs)->i);
}

static inline Value *idris2_lte_Integer(Value *lhs, Value *rhs) {
  return idris2_mkBool(((Value_Integer *)lhs)->i <= ((Value_Integer *)rhs)->i);
}

static inline Value *idris2_gt_Integer(Value *lhs, Value *rhs) {
  return idris2_mkBool(((Value_Integer *)lhs)->i > ((Value_Integer *)rhs)->i);
}

static inline Value *idris2_gte_Integer(Value *lhs, Value *rhs) {
  return idris2_mkBool(((Value_Integer *)lhs)->i >= ((Value_Integer *)rhs)->i);
}
