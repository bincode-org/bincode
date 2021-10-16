mod utils;

use core::cell::{Cell, RefCell};
use core::ops::Bound;
use core::time::Duration;
use utils::the_same;

#[test]
fn test_numbers() {
    // integer types
    the_same(5u8);
    the_same(5u16);
    the_same(5u32);
    the_same(5u64);
    the_same(5u128);
    the_same(5usize);

    the_same(5i8);
    the_same(5i16);
    the_same(5i32);
    the_same(5i64);
    the_same(5i128);
    the_same(5isize);

    the_same(5.0f32);
    the_same(5.0f64);

    // bool
    the_same(true);
    the_same(false);

    // utf8 characters
    for char in "aÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö文".chars()
    {
        the_same(char);
    }

    // tuples, up to 8
    the_same((1u8,));
    the_same((1u8, 2u8));
    the_same((1u8, 2u8, 3u8));
    the_same((1u8, 2u8, 3u8, 4u8));
    the_same((1u8, 2u8, 3u8, 4u8, 5u8));
    the_same((1u8, 2u8, 3u8, 4u8, 5u8, 6u8));
    the_same((1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8));
    the_same((1u8, 2u8, 3u8, 4u8, 5u8, 6u8, 7u8, 8u8));

    // arrays
    #[rustfmt::skip]
    the_same([
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
        33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
        65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
        81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96,
        97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128,
        129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144,
        145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160,
        161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176,
        177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
        193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208,
        209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224,
        225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240,
        241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255
    ]);

    // Common types
    the_same(Option::<u32>::None);
    the_same(Option::<u32>::Some(1234));

    the_same(Result::<u32, u8>::Ok(1555));
    the_same(Result::<u32, u8>::Err(15));

    the_same(Cell::<u32>::new(15));
    the_same(RefCell::<u32>::new(15));

    the_same(Duration::new(5, 730023852));
    the_same(5u8..10u8);
    the_same(5u8..=10u8);
    the_same(Bound::<usize>::Unbounded);
    the_same(Bound::<usize>::Included(105));
    the_same(Bound::<usize>::Excluded(5));
}

#[test]
fn test_refcell_already_borrowed() {
    let cell = RefCell::new(5u32);
    // first get a mutable reference to the cell
    let _mutable_guard = cell.borrow_mut();
    // now try to encode it
    let mut slice = [0u8; 10];
    let result = bincode::encode_into_slice(&cell, &mut slice)
        .expect_err("Encoding a borrowed refcell should fail");

    match result {
        bincode::error::EncodeError::RefCellAlreadyBorrowed { .. } => {} // ok
        x => panic!("Expected a RefCellAlreadyBorrowed error, found {:?}", x),
    }
}

#[test]
fn test_slice() {
    let mut buffer = [0u8; 32];
    let input: &[u8] = &[1, 2, 3, 4, 5, 6, 7];
    bincode::encode_into_slice(input, &mut buffer).unwrap();
    assert_eq!(&buffer[..8], &[7, 1, 2, 3, 4, 5, 6, 7]);

    let output: &[u8] = bincode::decode(&mut buffer[..8]).unwrap();
    assert_eq!(input, output);
}

#[test]
fn test_str() {
    let mut buffer = [0u8; 32];
    let input: &str = "Hello world";
    bincode::encode_into_slice(input, &mut buffer).unwrap();
    assert_eq!(
        &buffer[..12],
        &[11, 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]
    );

    let output: &str = bincode::decode(&mut buffer[..12]).unwrap();
    assert_eq!(input, output);
}

#[test]
fn test_array() {
    let mut buffer = [0u8; 32];
    let input: [u8; 10] = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    bincode::encode_into_slice(input, &mut buffer).unwrap();
    assert_eq!(&buffer[..10], &[10, 20, 30, 40, 50, 60, 70, 80, 90, 100]);

    let output: [u8; 10] = bincode::decode(&mut buffer[..10]).unwrap();
    assert_eq!(input, output);
}
