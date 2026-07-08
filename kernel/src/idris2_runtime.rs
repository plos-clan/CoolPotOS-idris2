#![allow(dead_code, non_snake_case)]

use core::ffi::{c_char, c_void};
use core::mem::{align_of, size_of, transmute};
use core::ptr::{self, addr_of_mut};

use crate::allocator;
use crate::serial;

const NO_TAG: u8 = 0;
const BITS64_TAG: u8 = 4;
const INT64_TAG: u8 = 8;
const INTEGER_TAG: u8 = 9;
const DOUBLE_TAG: u8 = 10;
const STRING_TAG: u8 = 12;
const CLOSURE_TAG: u8 = 15;
const CONSTRUCTOR_TAG: u8 = 17;
const IOREF_TAG: u8 = 20;
const ARRAY_TAG: u8 = 21;
const POINTER_TAG: u8 = 22;

const IMMORTAL_REFCOUNT: u16 = u16::MAX;
const UNBOXED_SHIFT: usize = 32;

type RawClosureFn = unsafe extern "C" fn() -> *mut Value;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueHeader {
    pub ref_counter: u16,
    pub tag: u8,
    pub reserved: u8,
}

#[repr(C)]
pub struct Value {
    pub header: ValueHeader,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueBits64 {
    pub header: ValueHeader,
    pub ui64: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueInt64 {
    pub header: ValueHeader,
    pub i64: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ValueInteger {
    pub header: ValueHeader,
    pub i: i64,
}

#[repr(C)]
pub struct ValueDouble {
    pub header: ValueHeader,
    pub d: f64,
}

#[repr(C)]
pub struct ValueString {
    pub header: ValueHeader,
    pub str_ptr: *mut c_char,
}

#[repr(C)]
pub struct ValueConstructor {
    pub header: ValueHeader,
    pub total: i32,
    pub tag: i32,
    pub name: *const c_char,
    pub args: [*mut Value; 0],
}

#[repr(C)]
pub struct ValueClosure {
    pub header: ValueHeader,
    pub f: *const c_void,
    pub arity: u8,
    pub filled: u8,
    pub args: [*mut Value; 0],
}

#[repr(C)]
pub struct ValueIORef {
    pub header: ValueHeader,
    pub value: *mut Value,
}

#[repr(C)]
pub struct ValuePointer {
    pub header: ValueHeader,
    pub ptr: *mut c_void,
}

#[repr(C)]
pub struct ValueArray {
    pub header: ValueHeader,
    pub capacity: i32,
    pub items: *mut *mut Value,
}

const fn immortal_header(tag: u8) -> ValueHeader {
    ValueHeader {
        ref_counter: IMMORTAL_REFCOUNT,
        tag,
        reserved: 0,
    }
}

const fn heap_header(tag: u8) -> ValueHeader {
    ValueHeader {
        ref_counter: 1,
        tag,
        reserved: 0,
    }
}

const fn int64_value(value: i64) -> ValueInt64 {
    ValueInt64 {
        header: immortal_header(INT64_TAG),
        i64: value,
    }
}

const fn bits64_value(value: u64) -> ValueBits64 {
    ValueBits64 {
        header: immortal_header(BITS64_TAG),
        ui64: value,
    }
}

const fn integer_value(value: i64) -> ValueInteger {
    ValueInteger {
        header: immortal_header(INTEGER_TAG),
        i: value,
    }
}

const fn build_predefined_int64() -> [ValueInt64; 100] {
    let mut values = [int64_value(0); 100];
    let mut index = 0;
    while index < 100 {
        values[index] = int64_value(index as i64);
        index += 1;
    }
    values
}

const fn build_predefined_bits64() -> [ValueBits64; 100] {
    let mut values = [bits64_value(0); 100];
    let mut index = 0;
    while index < 100 {
        values[index] = bits64_value(index as u64);
        index += 1;
    }
    values
}

const fn build_predefined_integer() -> [ValueInteger; 100] {
    let mut values = [integer_value(0); 100];
    let mut index = 0;
    while index < 100 {
        values[index] = integer_value(index as i64);
        index += 1;
    }
    values
}

#[unsafe(no_mangle)]
pub static idris2_predefined_Int64: [ValueInt64; 100] = build_predefined_int64();

#[unsafe(no_mangle)]
pub static idris2_predefined_Bits64: [ValueBits64; 100] = build_predefined_bits64();

#[unsafe(no_mangle)]
pub static idris2_predefined_Integer: [ValueInteger; 100] = build_predefined_integer();

#[inline]
fn halt_forever() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[inline]
fn is_unboxed(value: *mut Value) -> bool {
    ((value as usize) & 1) != 0
}

#[inline]
fn value_as_i64(value: *mut Value) -> i64 {
    (((value as usize) >> UNBOXED_SHIFT) as u32 as i32) as i64
}

#[inline]
unsafe fn closure_args(closure: *mut ValueClosure) -> *mut *mut Value {
    unsafe { addr_of_mut!((*closure).args).cast::<*mut Value>() }
}

#[inline]
unsafe fn constructor_args(constructor: *mut ValueConstructor) -> *mut *mut Value {
    unsafe { addr_of_mut!((*constructor).args).cast::<*mut Value>() }
}

unsafe fn alloc_or_halt(size: usize, align: usize) -> *mut u8 {
    let ptr = unsafe { allocator::alloc_zeroed(size, align) };
    if ptr.is_null() {
        halt_forever();
    }
    ptr
}

fn predefined_int64(value: i64) -> Option<*mut Value> {
    if (0..100).contains(&value) {
        Some(
            (&idris2_predefined_Int64[value as usize] as *const ValueInt64)
                .cast_mut()
                .cast(),
        )
    } else {
        None
    }
}

fn predefined_bits64(value: u64) -> Option<*mut Value> {
    if value < 100 {
        Some(
            (&idris2_predefined_Bits64[value as usize] as *const ValueBits64)
                .cast_mut()
                .cast(),
        )
    } else {
        None
    }
}

fn predefined_integer(value: i64) -> Option<*mut Value> {
    if (0..100).contains(&value) {
        Some(
            (&idris2_predefined_Integer[value as usize] as *const ValueInteger)
                .cast_mut()
                .cast(),
        )
    } else {
        None
    }
}

unsafe fn alloc_boxed<T>(value: T) -> *mut T {
    let ptr = unsafe { alloc_or_halt(size_of::<T>(), align_of::<T>()) }.cast::<T>();
    unsafe { ptr.write(value) };
    ptr
}

unsafe fn dealloc_boxed<T>(ptr: *mut T) {
    unsafe { allocator::dealloc(ptr.cast(), size_of::<T>(), align_of::<T>()) };
}

unsafe fn dealloc_c_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    let mut len = 0usize;
    unsafe {
        while *ptr.add(len).cast::<u8>() != 0 {
            len += 1;
        }
    }

    unsafe { allocator::dealloc(ptr.cast(), len + 1, align_of::<u8>()) };
}

unsafe fn dealloc_value_storage(value: *mut Value) {
    unsafe {
        match (*value).header.tag {
            BITS64_TAG => dealloc_boxed(value.cast::<ValueBits64>()),
            INT64_TAG => dealloc_boxed(value.cast::<ValueInt64>()),
            INTEGER_TAG => dealloc_boxed(value.cast::<ValueInteger>()),
            DOUBLE_TAG => dealloc_boxed(value.cast::<ValueDouble>()),
            STRING_TAG => {
                let string = value.cast::<ValueString>();
                dealloc_c_string((*string).str_ptr);
                dealloc_boxed(string);
            }
            CONSTRUCTOR_TAG => {
                let constructor = value.cast::<ValueConstructor>();
                allocator::dealloc(
                    constructor.cast(),
                    size_of::<ValueConstructor>()
                        + ((*constructor).total as usize * size_of::<*mut Value>()),
                    align_of::<ValueConstructor>(),
                );
            }
            CLOSURE_TAG => {
                let closure = value.cast::<ValueClosure>();
                allocator::dealloc(
                    closure.cast(),
                    size_of::<ValueClosure>()
                        + ((*closure).filled as usize * size_of::<*mut Value>()),
                    align_of::<ValueClosure>(),
                );
            }
            IOREF_TAG => dealloc_boxed(value.cast::<ValueIORef>()),
            ARRAY_TAG => {
                let array = value.cast::<ValueArray>();
                if !(*array).items.is_null() && (*array).capacity > 0 {
                    allocator::dealloc(
                        (*array).items.cast(),
                        (*array).capacity as usize * size_of::<*mut Value>(),
                        align_of::<*mut Value>(),
                    );
                }
                dealloc_boxed(array);
            }
            POINTER_TAG => dealloc_boxed(value.cast::<ValuePointer>()),
            NO_TAG => {}
            _ => halt_forever(),
        }
    }
}

unsafe fn stored_closure_fn(closure: *mut ValueClosure) -> RawClosureFn {
    unsafe { transmute((*closure).f) }
}

unsafe fn invoke_closure(closure: *mut ValueClosure) -> *mut Value {
    let args = unsafe { closure_args(closure) };

    type ValuePtr = *mut Value;

    unsafe {
        match (*closure).arity {
            0 => {
                let function: unsafe extern "C" fn() -> *mut Value = transmute((*closure).f);
                function()
            }
            1 => {
                let function: unsafe extern "C" fn(ValuePtr) -> *mut Value =
                    transmute((*closure).f);
                function(*args.add(0))
            }
            2 => {
                let function: unsafe extern "C" fn(ValuePtr, ValuePtr) -> *mut Value =
                    transmute((*closure).f);
                function(*args.add(0), *args.add(1))
            }
            3 => {
                let function: unsafe extern "C" fn(ValuePtr, ValuePtr, ValuePtr) -> *mut Value =
                    transmute((*closure).f);
                function(*args.add(0), *args.add(1), *args.add(2))
            }
            4 => {
                let function: unsafe extern "C" fn(
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                ) -> *mut Value = transmute((*closure).f);
                function(*args.add(0), *args.add(1), *args.add(2), *args.add(3))
            }
            5 => {
                let function: unsafe extern "C" fn(
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                ) -> *mut Value = transmute((*closure).f);
                function(
                    *args.add(0),
                    *args.add(1),
                    *args.add(2),
                    *args.add(3),
                    *args.add(4),
                )
            }
            6 => {
                let function: unsafe extern "C" fn(
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                ) -> *mut Value = transmute((*closure).f);
                function(
                    *args.add(0),
                    *args.add(1),
                    *args.add(2),
                    *args.add(3),
                    *args.add(4),
                    *args.add(5),
                )
            }
            7 => {
                let function: unsafe extern "C" fn(
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                ) -> *mut Value = transmute((*closure).f);
                function(
                    *args.add(0),
                    *args.add(1),
                    *args.add(2),
                    *args.add(3),
                    *args.add(4),
                    *args.add(5),
                    *args.add(6),
                )
            }
            8 => {
                let function: unsafe extern "C" fn(
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                    ValuePtr,
                ) -> *mut Value = transmute((*closure).f);
                function(
                    *args.add(0),
                    *args.add(1),
                    *args.add(2),
                    *args.add(3),
                    *args.add(4),
                    *args.add(5),
                    *args.add(6),
                    *args.add(7),
                )
            }
            _ => halt_forever(),
        }
    }
}

unsafe fn consume_full_closure(closure: *mut ValueClosure) -> *mut Value {
    let result = unsafe { invoke_closure(closure) };
    unsafe { dealloc_value_storage(closure.cast()) };
    result
}

unsafe fn release_children(value: *mut Value) {
    unsafe {
        match (*value).header.tag {
            CONSTRUCTOR_TAG => {
                let constructor = value.cast::<ValueConstructor>();
                let args = constructor_args(constructor);
                let mut index = 0;
                while index < (*constructor).total as usize {
                    idris2_removeReference(*args.add(index));
                    index += 1;
                }
            }
            CLOSURE_TAG => {
                let closure = value.cast::<ValueClosure>();
                let args = closure_args(closure);
                let mut index = 0;
                while index < (*closure).filled as usize {
                    idris2_removeReference(*args.add(index));
                    index += 1;
                }
            }
            IOREF_TAG => {
                let ioref = value.cast::<ValueIORef>();
                idris2_removeReference((*ioref).value);
            }
            ARRAY_TAG => {
                let array = value.cast::<ValueArray>();
                let mut index = 0;
                while index < (*array).capacity as usize {
                    let item = *(*array).items.add(index);
                    idris2_removeReference(item);
                    index += 1;
                }
            }
            _ => {}
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_missing_ffi() {
    halt_forever();
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_newValue(size: usize) -> *mut Value {
    unsafe { alloc_or_halt(size, align_of::<usize>()).cast() }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_newReference(source: *mut Value) -> *mut Value {
    if source.is_null() || is_unboxed(source) {
        return source;
    }

    unsafe {
        let header = &mut (*source).header;
        if header.ref_counter != IMMORTAL_REFCOUNT && header.ref_counter < IMMORTAL_REFCOUNT {
            header.ref_counter = header.ref_counter.saturating_add(1);
        }
    }

    source
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_removeReference(source: *mut Value) {
    if source.is_null() || is_unboxed(source) {
        return;
    }

    unsafe {
        let header = &mut (*source).header;
        if header.ref_counter == IMMORTAL_REFCOUNT {
            return;
        }

        if header.ref_counter > 1 {
            header.ref_counter -= 1;
            return;
        }

        header.ref_counter = 0;
        release_children(source);
        dealloc_value_storage(source);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_removeReuseConstructor(_constr: *mut ValueConstructor) {}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_newConstructor(total: i32, tag: i32) -> *mut ValueConstructor {
    let size = size_of::<ValueConstructor>() + (total as usize * size_of::<*mut Value>());
    let constructor =
        unsafe { alloc_or_halt(size, align_of::<ValueConstructor>()) }.cast::<ValueConstructor>();

    unsafe {
        ptr::write(
            constructor,
            ValueConstructor {
                header: heap_header(CONSTRUCTOR_TAG),
                total,
                tag,
                name: ptr::null(),
                args: [],
            },
        );
    }

    constructor
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_mkClosure(f: RawClosureFn, arity: u8, filled: u8) -> *mut ValueClosure {
    let size = size_of::<ValueClosure>() + (filled as usize * size_of::<*mut Value>());
    let closure = unsafe { alloc_or_halt(size, align_of::<ValueClosure>()) }.cast::<ValueClosure>();

    unsafe {
        ptr::write(
            closure,
            ValueClosure {
                header: heap_header(CLOSURE_TAG),
                f: f as *const () as *const c_void,
                arity,
                filled,
                args: [],
            },
        );
    }

    closure
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_mkBits64(value: u64) -> *mut Value {
    if let Some(predefined) = predefined_bits64(value) {
        return predefined;
    }

    let boxed = unsafe {
        alloc_boxed(ValueBits64 {
            header: heap_header(BITS64_TAG),
            ui64: value,
        })
    };
    boxed.cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_mkInt64(value: i64) -> *mut Value {
    if let Some(predefined) = predefined_int64(value) {
        return predefined;
    }

    let boxed = unsafe {
        alloc_boxed(ValueInt64 {
            header: heap_header(INT64_TAG),
            i64: value,
        })
    };
    boxed.cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_mkDouble(value: f64) -> *mut Value {
    let boxed = unsafe {
        alloc_boxed(ValueDouble {
            header: heap_header(DOUBLE_TAG),
            d: value,
        })
    };
    boxed.cast()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_mkIntegerLiteral(text: *const c_char) -> *mut Value {
    if text.is_null() {
        return idris2_getPredefinedInteger(0);
    }

    let mut negative = false;
    let mut cursor = text.cast::<u8>();
    let mut value: i64 = 0;

    unsafe {
        if *cursor == b'-' {
            negative = true;
            cursor = cursor.add(1);
        }

        while *cursor != 0 {
            let digit = *cursor;
            if !digit.is_ascii_digit() {
                halt_forever();
            }

            value = value
                .saturating_mul(10)
                .saturating_add((digit - b'0') as i64);
            cursor = cursor.add(1);
        }
    }

    let value = if negative { -value } else { value };
    idris2_getPredefinedInteger(value)
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_getPredefinedInteger(value: i64) -> *mut Value {
    if let Some(predefined) = predefined_integer(value) {
        return predefined;
    }

    let boxed = unsafe {
        alloc_boxed(ValueInteger {
            header: heap_header(INTEGER_TAG),
            i: value,
        })
    };
    boxed.cast()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_add_Integer(lhs: *mut Value, rhs: *mut Value) -> *mut Value {
    let value = unsafe { (*(lhs.cast::<ValueInteger>())).i + (*(rhs.cast::<ValueInteger>())).i };
    idris2_getPredefinedInteger(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_sub_Integer(lhs: *mut Value, rhs: *mut Value) -> *mut Value {
    let value = unsafe { (*(lhs.cast::<ValueInteger>())).i - (*(rhs.cast::<ValueInteger>())).i };
    idris2_getPredefinedInteger(value)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_cast_Integer_to_Int64(value: *mut Value) -> *mut Value {
    let integer = unsafe { (*(value.cast::<ValueInteger>())).i };
    idris2_mkInt64(integer)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_apply_closure(closure: *mut Value, arg: *mut Value) -> *mut Value {
    if closure.is_null() {
        halt_forever();
    }

    if is_unboxed(closure) {
        halt_forever();
    }

    let closure = closure.cast::<ValueClosure>();

    unsafe {
        let filled = (*closure).filled as usize;
        let next_filled = filled + 1;
        let applied = idris2_mkClosure(
            stored_closure_fn(closure),
            (*closure).arity,
            next_filled as u8,
        );
        let src_args = closure_args(closure);
        let dst_args = closure_args(applied);

        let mut index = 0;
        while index < filled {
            *dst_args.add(index) = idris2_newReference(*src_args.add(index));
            index += 1;
        }
        *dst_args.add(filled) = arg;

        idris2_removeReference(closure.cast());

        if next_filled < (*applied).arity as usize {
            applied.cast()
        } else {
            idris2_trampoline(consume_full_closure(applied))
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_tailcall_apply_closure(
    closure: *mut Value,
    arg: *mut Value,
) -> *mut Value {
    unsafe { idris2_apply_closure(closure, arg) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_trampoline(mut value: *mut Value) -> *mut Value {
    loop {
        if value.is_null() || is_unboxed(value) {
            return value;
        }

        unsafe {
            if (*value).header.tag != CLOSURE_TAG {
                return value;
            }

            let closure = value.cast::<ValueClosure>();
            if (*closure).filled != (*closure).arity {
                return value;
            }

            value = consume_full_closure(closure);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_extractInt(value: *mut Value) -> i64 {
    if value.is_null() {
        return 0;
    }

    if is_unboxed(value) {
        return value_as_i64(value);
    }

    unsafe {
        match (*value).header.tag {
            INT64_TAG => (*(value.cast::<ValueInt64>())).i64,
            INTEGER_TAG => (*(value.cast::<ValueInteger>())).i,
            BITS64_TAG => (*(value.cast::<ValueBits64>())).ui64 as i64,
            DOUBLE_TAG => (*(value.cast::<ValueDouble>())).d as i64,
            NO_TAG => 0,
            _ => halt_forever(),
        }
    }
}

pub unsafe fn extract_bits64(value: *mut Value) -> u64 {
    if value.is_null() {
        return 0;
    }

    if is_unboxed(value) {
        return value_as_i64(value) as u64;
    }

    unsafe {
        match (*value).header.tag {
            BITS64_TAG => (*(value.cast::<ValueBits64>())).ui64,
            INT64_TAG => (*(value.cast::<ValueInt64>())).i64 as u64,
            INTEGER_TAG => (*(value.cast::<ValueInteger>())).i as u64,
            NO_TAG => 0,
            _ => halt_forever(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_dumpMemoryStats() {}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_isNull(ptr: *mut c_void) -> i32 {
    ptr.is_null() as i32
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_getNull() -> *mut c_void {
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_getString(ptr: *mut c_void) -> *mut c_char {
    ptr.cast()
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_getErrno() -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_strerror(_errnum: i32) -> *mut c_char {
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_getStr() -> *mut c_char {
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn idris2_putStr(text: *mut c_char) {
    unsafe { serial::write_c_str(text.cast_const()) };
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_sleep(_sec: i32) {}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_usleep(_usec: i32) {}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_time() -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_getArgCount() -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_setArgs(_argc: i32, _argv: *mut *mut c_char) {}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_getArg(_index: i32) -> *mut c_char {
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_getEnvPair(_index: i32) -> *mut c_char {
    ptr::null_mut()
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_getPID() -> i32 {
    0
}

#[unsafe(no_mangle)]
pub extern "C" fn idris2_getNProcessors() -> i64 {
    1
}
