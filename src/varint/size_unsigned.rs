use super::SINGLE_BYTE_MAX;

pub fn varint_size_u16(val: u16) -> usize {
    if val <= SINGLE_BYTE_MAX as _ {
        1
    } else {
        1 + std::mem::size_of::<u16>()
    }
}

pub fn varint_size_u32(val: u32) -> usize {
    if val <= SINGLE_BYTE_MAX as _ {
        1
    } else if val <= u16::MAX as _ {
        1 + std::mem::size_of::<u16>()
    } else {
        1 + std::mem::size_of::<u32>()
    }
}

pub fn varint_size_u64(val: u64) -> usize {
    if val <= SINGLE_BYTE_MAX as _ {
        1
    } else if val <= u16::MAX as _ {
        1 + std::mem::size_of::<u16>()
    } else if val <= u32::MAX as _ {
        1 + std::mem::size_of::<u32>()
    } else {
        1 + std::mem::size_of::<u64>()
    }
}

pub fn varint_size_u128(val: u128) -> usize {
    if val <= SINGLE_BYTE_MAX as _ {
        1
    } else if val <= u16::MAX as _ {
        1 + std::mem::size_of::<u16>()
    } else if val <= u32::MAX as _ {
        1 + std::mem::size_of::<u32>()
    } else if val <= u64::MAX as _ {
        1 + std::mem::size_of::<u64>()
    } else {
        1 + std::mem::size_of::<u128>()
    }
}

pub fn varint_size_usize(val: usize) -> usize {
    // usize is being encoded as a u64
    varint_size_u64(val as u64)
}

#[test]
fn test_size_u16() {
    // these should all encode to a single byte
    for i in 0u16..=SINGLE_BYTE_MAX as u16 {
        assert_eq!(varint_size_u16(i), 1, "value: {}", i);
    }

    // these values should encode in 3 bytes (leading byte + 2 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        SINGLE_BYTE_MAX as u16 + 1,
        300,
        500,
        700,
        888,
        1234,
        u16::MAX,
    ] {
        assert_eq!(varint_size_u16(i), 3, "value: {}", i);
    }
}

#[test]
fn test_size_u32() {
    // these should all encode to a single byte
    for i in 0u32..=SINGLE_BYTE_MAX as u32 {
        assert_eq!(varint_size_u32(i), 1, "value: {}", i);
    }

    // these values should encode in 3 bytes (leading byte + 2 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        SINGLE_BYTE_MAX as u32 + 1,
        300,
        500,
        700,
        888,
        1234,
        u16::MAX as u32,
    ] {
        assert_eq!(varint_size_u32(i), 3, "value: {}", i);
    }

    // these values should encode in 5 bytes (leading byte + 4 bytes)
    // Values chosen at random, add new cases as needed
    for i in [u16::MAX as u32 + 1, 100_000, 1_000_000, u32::MAX] {
        assert_eq!(varint_size_u32(i), 5, "value: {}", i);
    }
}

#[test]
fn test_size_u64() {
    // these should all encode to a single byte
    for i in 0u64..=SINGLE_BYTE_MAX as u64 {
        assert_eq!(varint_size_u64(i), 1, "value: {}", i);
    }

    // these values should encode in 3 bytes (leading byte + 2 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        SINGLE_BYTE_MAX as u64 + 1,
        300,
        500,
        700,
        888,
        1234,
        u16::MAX as u64,
    ] {
        assert_eq!(varint_size_u64(i), 3, "value: {}", i);
    }

    // these values should encode in 5 bytes (leading byte + 4 bytes)
    // Values chosen at random, add new cases as needed
    for i in [u16::MAX as u64 + 1, 100_000, 1_000_000, u32::MAX as u64] {
        assert_eq!(varint_size_u64(i), 5, "value: {}", i);
    }

    // these values should encode in 9 bytes (leading byte + 8 bytes)
    // Values chosen at random, add new cases as needed
    for i in [u32::MAX as u64 + 1, 5_000_000_000, u64::MAX] {
        assert_eq!(varint_size_u64(i), 9, "value: {}", i);
    }
}

#[test]
fn test_size_u128() {
    // these should all encode to a single byte
    for i in 0u128..=SINGLE_BYTE_MAX as u128 {
        assert_eq!(varint_size_u128(i), 1, "value: {}", i);
    }

    // these values should encode in 3 bytes (leading byte + 2 bytes)
    // Values chosen at random, add new cases as needed
    for i in [
        SINGLE_BYTE_MAX as u128 + 1,
        300,
        500,
        700,
        888,
        1234,
        u16::MAX as u128,
    ] {
        assert_eq!(varint_size_u128(i), 3, "value: {}", i);
    }

    // these values should encode in 5 bytes (leading byte + 4 bytes)
    // Values chosen at random, add new cases as needed
    for i in [u16::MAX as u128 + 1, 100_000, 1_000_000, u32::MAX as u128] {
        assert_eq!(varint_size_u128(i), 5, "value: {}", i);
    }

    // these values should encode in 9 bytes (leading byte + 8 bytes)
    // Values chosen at random, add new cases as needed
    for i in [u32::MAX as u128 + 1, 5_000_000_000, u64::MAX as u128] {
        assert_eq!(varint_size_u128(i), 9, "value: {}", i);
    }

    // these values should encode in 17 bytes (leading byte + 16 bytes)
    // Values chosen at random, add new cases as needed
    for i in [u64::MAX as u128 + 1, u128::MAX] {
        assert_eq!(varint_size_u128(i), 17, "value: {}", i);
    }
}
