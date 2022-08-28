// Takes 4 bytes and pack them as a u32
pub fn as_u32(data: &[u8]) -> u32 {
    data.iter()
        .take(4)
        .enumerate()
        .map(|(i, d)| (*d as u32) << (i * 8))
        .sum::<u32>()
}

// Takes 8 bytes and pack them as a u64
pub fn as_u64(data: &[u8]) -> u64 {
    data.iter()
        .take(8)
        .enumerate()
        .map(|(i, d)| (*d as u64) << (i * 8))
        .sum::<u64>()
}

// Converts the 27 most significant bits of the u32 to a signed integer over 27 bits
// Used to read bit field of number headers
pub fn as_i27(raw_value: u32) -> i32 {
    let mask = u32::MAX & !0b11111;
    let bit_field = (raw_value & mask) >> 5;
    let sign_mask = 0b1 << 26;
    let is_negative_value = bit_field & sign_mask != 0;

    if is_negative_value {
        -1 * (((!bit_field) + 1) % (2_u32.pow(26))) as i32
    } else {
        bit_field as i32
    }
}

// Converts the 27 most significant bits of the u32 to an unsigned integer over 27 bits
// Used to read bit field of number headers
pub fn as_u27(raw_value: u32) -> u32 {
    let mask = u32::MAX & !0b11111;
    (raw_value & mask) >> 5
}
