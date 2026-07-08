use core::arch::asm;
use core::ffi::c_char;

#[inline]
pub fn write_byte(byte: u8) {
    unsafe {
        asm!(
            "ecall",
            inout("a0") byte as usize => _,
            inout("a7") 1usize => _,
        );
    }
}

pub fn write_str(message: &str) {
    for byte in message.bytes() {
        write_byte(byte);
    }
}

pub fn write_hex_u64(value: u64) {
    write_str("0x");

    for shift in (0..16).rev() {
        let nibble = ((value >> (shift * 4)) & 0xf) as u8;
        let digit = match nibble {
            0..=9 => b'0' + nibble,
            _ => b'a' + (nibble - 10),
        };
        write_byte(digit);
    }
}

/// # Safety
/// Please be sure that message is a non-empty pointer, and that it ends with null.
pub unsafe fn write_c_str(message: *const c_char) {
    if message.is_null() {
        return;
    }

    let mut cursor = message.cast::<u8>();
    let mut previous = 0u8;

    unsafe {
        while *cursor != 0 {
            let byte = *cursor;
            if byte == b'\n' && previous != b'\r' {
                write_byte(b'\r');
            }
            write_byte(byte);
            previous = byte;
            cursor = cursor.add(1);
        }
    }
}
