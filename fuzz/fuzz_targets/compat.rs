#![no_main]
use libfuzzer_sys::fuzz_target;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ffi::CString;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{NonZeroI128, NonZeroI32, NonZeroU128, NonZeroU32};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

#[derive(bincode::Decode, bincode::Encode, PartialEq, Debug, serde::Serialize, serde::Deserialize, Eq, PartialOrd, Ord)]
enum AllTypes {
    BTreeMap(BTreeMap<u8, AllTypes>),
    BTreeSet(BTreeSet<AllTypes>),
    VecDeque(VecDeque<AllTypes>),
    Vec(Vec<u8>),
    String(String),
    Box(Box<u8>),
    BoxSlice(Box<[u8]>),
    CString(CString),
    SystemTime(SystemTime),
    Duration(Duration),
    PathBuf(PathBuf),
    IpAddr(IpAddr),
    Ipv4Addr(Ipv4Addr),
    Ipv6Addr(Ipv6Addr),
    SocketAddr(SocketAddr),
    SocketAddrV4(SocketAddrV4),
    SocketAddrV6(SocketAddrV6),
    NonZeroU32(NonZeroU32),
    NonZeroI32(NonZeroI32),
    NonZeroU128(NonZeroU128),
    NonZeroI128(NonZeroI128),
    I128(i128),
    I8(i8),
    U128(u128),
    U8(u8),
    // Cow(Cow<'static, [u8]>), Blocked, see comment on decode
}

fuzz_target!(|data: &[u8]| {
    let config = bincode::config::legacy().with_limit::<1024>();
    #[allow(deprecated)]
    let mut configv1 = bincodev1::config();
    configv1.limit(1024);
    let bincode_v1: Result<AllTypes, _> = configv1.deserialize(data);
    let bincode_v2: Result<(AllTypes, _), _> = bincode::decode_from_slice(data, config);

    if bincode_v1.as_ref().ok() != bincode_v2.as_ref().ok().map(|x| &x.0) {
        println!("Bytes:      {:?}", data);
        println!("Bincode V1: {:?}", bincode_v1);
        println!("Bincode V2: {:?}", bincode_v2);
        panic!("failed equality check");
    }
});
