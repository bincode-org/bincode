#[test]
fn test() {
    super::test_same((1,));
    super::test_same(TupleS(2.0, 3.0, 4.0));
    super::test_same(Option::<u32>::Some(5));
    super::test_same(Option::<u32>::None);
    super::test_same(Result::<u32, u8>::Ok(5));
    super::test_same(Result::<u32, u8>::Err(5));
    super::test_same(std::net::Ipv4Addr::LOCALHOST);
    super::test_same(std::net::Ipv6Addr::LOCALHOST);
}

#[test]
fn test_arrays() {
    // serde is known to be weird with arrays
    // Arrays of length 32 and less are encoded as tuples, but arrays 33 and up are encoded as slices
    // we need to make sure we're compatible with this
    super::test_same([0u8; 0]);
    super::test_same([1u8; 1]);
    super::test_same([1u8, 2]);
    super::test_same([1u8, 2, 3]);
    super::test_same([1u8, 2, 3, 4]);
    super::test_same([1u8, 2, 3, 4, 5]);
    super::test_same([1u8, 2, 3, 4, 5, 6]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7, 8]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7, 8, 9]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    super::test_same([1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31,
    ]);
    super::test_same([
        1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32,
    ]);
}

#[derive(
    bincode_2::Encode, bincode_2::Decode, serde::Serialize, serde::Deserialize, Debug, PartialEq,
)]
#[bincode(crate = "bincode_2")]
struct TupleS(f32, f32, f32);
