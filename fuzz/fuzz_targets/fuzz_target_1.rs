#![no_main]
use libfuzzer_sys::fuzz_target;

use std::borrow::Cow;
use std::collections::*;
use std::rc::Rc;
use std::sync::Arc;
use std::ffi::CString;
use std::num::*;
use std::net::*;
use std::path::*;
use std::time::*;

#[derive(bincode::Decode, bincode::Encode, PartialEq, Debug)]
enum AllTypes {
    BTreeMap(BTreeMap<u8, u8>),
    HashMap(HashMap<u8, u8>),
    BTreeSet(BTreeSet<u8>),
    VecDeque(VecDeque<u8>),
    Vec(VecDeque<u8>),
    String(String),
    Box(Box<u8>),
    BoxSlice(Box<[u8]>),
    Rc(Rc<u8>),
    Arc(Arc<u8>),
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
    let config = bincode::config::Configuration::standard().with_limit::<1024>();
    let result: Result<(AllTypes, _), _> = bincode::decode_from_slice(
        data,
        config,
    );

    if let Ok((before, _)) = result {
        let encoded = bincode::encode_to_vec(&before, config).expect("round trip");
        let (after, _) = bincode::decode_from_slice(&encoded, config).unwrap();
        assert_eq!(before, after);
    }
});
