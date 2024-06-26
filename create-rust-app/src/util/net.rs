use std::{
    net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6, TcpListener, ToSocketAddrs},
    ops::Range,
};

/// binds a [`TcpListener`] to the given [`addr`](`ToSocketAddrs`)
fn test_bind<A: ToSocketAddrs>(addr: A) -> bool {
    TcpListener::bind(addr)
        .map(|t| t.local_addr().is_ok())
        .unwrap_or(false)
}

/// is the given port free on this machine
#[must_use]
pub fn is_port_free(port: u16) -> bool {
    let ipv4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let ipv6 = SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, port, 0, 0);

    test_bind(ipv6) && test_bind(ipv4)
}

/// max range is 65535
#[must_use]
pub fn find_free_port(mut range: Range<u16>) -> Option<u16> {
    range.find(|port| is_port_free(*port))
}
