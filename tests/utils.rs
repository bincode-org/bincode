use core::fmt::Debug;

fn the_same_with_config<V, C, CMP>(element: &V, config: C, cmp: CMP)
where
    V: TheSameTrait,
    C: bincode::config::Config,
    CMP: Fn(&V, &V) -> bool,
{
    let mut buffer = [0u8; 2048];
    let calculated_size = bincode::encoded_size(&element, config).unwrap();
    let len = bincode::encode_into_slice(&element, &mut buffer, config).unwrap();
    println!(
        "{:?} ({}): {:?} ({:?})",
        element,
        core::any::type_name::<V>(),
        &buffer[..len],
        core::any::type_name::<C>()
    );
    assert_eq!(
        calculated_size, len,
        "Calculated encoded size does not match actual encoded size\nCalculated: {:?}\nActual:     {:?}",
        calculated_size, len,
    );
    let (decoded, decoded_len): (V, usize) = bincode::decode_from_slice(&buffer, config).unwrap();

    assert!(
        cmp(element, &decoded),
        "Comparison failed\nDecoded:  {:?}\nExpected: {:?}\nBytes: {:?}",
        decoded,
        element,
        &buffer[..len],
    );
    assert_eq!(len, decoded_len);

    #[cfg(feature = "serde")]
    the_same_with_config_serde(element, config, cmp)
}

#[cfg(feature = "serde")]
fn the_same_with_config_serde<V, C, CMP>(element: &V, config: C, cmp: CMP)
where
    V: TheSameTrait,
    C: bincode::config::Config,
    CMP: Fn(&V, &V) -> bool,
{
    use bincode::error::EncodeError;

    let mut buffer = [0u8; 2048];
    let len = bincode::serde::encode_into_slice(&element, &mut buffer, config);

    let decoded = bincode::serde::decode_from_slice(&buffer, config);

    if !C::SKIP_FIXED_ARRAY_LENGTH {
        let len = len.unwrap();
        let (decoded, decoded_len): (V, usize) = decoded.unwrap();
        println!(
            "{:?} ({}): {:?} ({:?})",
            element,
            core::any::type_name::<V>(),
            &buffer[..len],
            core::any::type_name::<C>()
        );

        assert!(
            cmp(element, &decoded),
            "Comparison failed\nDecoded:  {:?}\nExpected: {:?}\nBytes: {:?}",
            decoded,
            element,
            &buffer[..len],
        );
        assert_eq!(len, decoded_len);
    } else {
        match len.unwrap_err() {
            EncodeError::Serde(bincode::serde::EncodeError::SkipFixedArrayLengthNotSupported) => {}
            err => panic!("Unexpected error: {:?}", err),
        }
    }
}

pub fn the_same_with_comparer<V, CMP>(element: V, cmp: CMP)
where
    V: TheSameTrait,
    CMP: Fn(&V, &V) -> bool,
{
    // A matrix of each different config option possible
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_little_endian()
            .with_variable_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_big_endian()
            .with_variable_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_little_endian()
            .with_variable_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_big_endian()
            .with_variable_int_encoding(),
        &cmp,
    );
}

#[cfg(feature = "serde")]
pub trait TheSameTrait:
    bincode::Encode
    + bincode::Decode
    + bincode::EncodedSize
    + serde::de::DeserializeOwned
    + serde::Serialize
    + Debug
    + 'static
{
}
#[cfg(feature = "serde")]
impl<T> TheSameTrait for T where
    T: bincode::Encode
        + bincode::Decode
        + bincode::EncodedSize
        + serde::de::DeserializeOwned
        + serde::Serialize
        + Debug
        + 'static
{
}

#[cfg(not(feature = "serde"))]
pub trait TheSameTrait:
    bincode::Encode + bincode::Decode + bincode::EncodedSize + Debug + 'static
{
}
#[cfg(not(feature = "serde"))]
impl<T> TheSameTrait for T where
    T: bincode::Encode + bincode::Decode + bincode::EncodedSize + Debug + 'static
{
}

#[allow(dead_code)] // This is not used in every test
pub fn the_same<V: TheSameTrait + PartialEq>(element: V) {
    the_same_with_comparer(element, |a, b| a == b);
}
