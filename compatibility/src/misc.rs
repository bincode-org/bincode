#[test]
fn test() {
    super::test_same((1,));
    super::test_same(Option::<u32>::Some(5));
    super::test_same(Option::<u32>::None);
    super::test_same(Result::<u32, u8>::Ok(5));
    super::test_same(Result::<u32, u8>::Err(5));
    super::test_same(std::net::Ipv4Addr::LOCALHOST);
    super::test_same(std::net::Ipv6Addr::LOCALHOST);
}
