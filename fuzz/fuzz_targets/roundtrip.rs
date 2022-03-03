#![no_main]
use libfuzzer_sys::fuzz_target;

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::ffi::CString;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
use std::num::{NonZeroI128, NonZeroI32, NonZeroU128, NonZeroU32};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

#[derive(bincode::Decode, bincode::Encode, PartialEq, Debug)]
enum AllTypes {
    BTreeMap(BTreeMap<u8, u8>),
    HashMap(HashMap<u8, u8>),
    HashSet(HashSet<u8>),
    BTreeSet(BTreeSet<u8>),
    VecDeque(VecDeque<AllTypes>),
    Vec(Vec<AllTypes>),
    String(String),
    Box(Box<AllTypes>),
    BoxSlice(Box<[AllTypes]>),
    Rc(Rc<AllTypes>),
    Arc(Arc<AllTypes>),
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
    // Cow(Cow<'static, [u8]>), Blocked, see comment on decode
}

fuzz_target!(|data: &[u8]| {
    let config = bincode::config::standard().with_limit::<1024>();
    let result: Result<(AllTypes, _), _> = bincode::decode_from_slice(data, config);

    if let Ok((before, _)) = result {
        let encoded = bincode::encode_to_vec(&before, config).expect("round trip");
        let (after, _) = bincode::decode_from_slice(&encoded, config).unwrap();
        assert_eq!(before, after);
    }
});
