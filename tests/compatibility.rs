#![cfg(all(feature = "serde", feature = "alloc"))]

use bincode1::Options;
use rand::Rng;

fn test_same_with_config<T, C, O>(t: &T, bincode_1_options: O, bincode_2_config: C)
	where T: bincode::Encode + serde_incl::Serialize + core::fmt::Debug,
	C: bincode::config::Config + Clone,
	O: bincode1::Options + Clone
{
	let bincode_1_output = bincode_1_options.clone().serialize(t).unwrap();
	let bincode_2_output = bincode::encode_to_vec(t, bincode_2_config.clone()).unwrap();

	assert_eq!(bincode_1_output, bincode_2_output, "{:?} serializes differently", t);
}
fn test_same<T>(t: T)
	where T: bincode::Encode + serde_incl::Serialize + core::fmt::Debug,
{
	test_same_with_config(
		&t,
		bincode1::options().with_big_endian().with_varint_encoding(),
	bincode::config::Configuration::legacy().with_big_endian().with_variable_int_encoding()
	);
	test_same_with_config(
		&t,
		bincode1::options().with_little_endian().with_varint_encoding(),
	bincode::config::Configuration::legacy().with_little_endian().with_variable_int_encoding()
	);
	test_same_with_config(
		&t,
		bincode1::options().with_big_endian().with_fixint_encoding(),
	bincode::config::Configuration::legacy().with_big_endian().with_fixed_int_encoding()
	);
	test_same_with_config(
		&t,
		bincode1::options().with_little_endian().with_fixint_encoding(),
	bincode::config::Configuration::legacy().with_little_endian().with_fixed_int_encoding()
	);
}

#[test]
fn rand() {
	// https://github.com/rust-random/rand/blob/19404d68764ed08513131f82157e2ccad69dcf83/rand_pcg/src/pcg64.rs#L37-L40
	#[derive(Debug, bincode::Encode, bincode::Decode, serde_derive::Serialize, serde_derive::Deserialize)]
	#[serde(crate = "serde_incl")]
	pub struct Lcg64Xsh32 {
		state: u64,
		increment: u64,
	}

	let mut rng = rand::thread_rng();
	for _ in 0..100 {
		test_same(Lcg64Xsh32 {
			state: rng.gen(),
			increment: rng.gen(),
		});
	}
}
