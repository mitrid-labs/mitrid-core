use std::net::{SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

use fixture::io::{Address, address::DEFAULT_SOCKET};

#[test]
fn test_address_string() {
    let addr_a = Address::new("address");
    let addr_b = Address::new(&addr_a.to_string());
    assert_eq!(addr_a, addr_b)
}

#[test]
fn test_address_ipaddr() {
    let port = 8080;

    let socketaddrv4_a = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 255)), port);
    let addrv4 = Address::from_socket(&socketaddrv4_a);

    let res = addrv4.to_socket();
    assert!(res.is_ok());

    let socketaddrv4_b = res.unwrap();
    assert_eq!(socketaddrv4_a, socketaddrv4_b);

    let socketaddrv6_a = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), port);
    let addrv6 = Address::from_socket(&socketaddrv6_a);

    let res = addrv6.to_socket();
    assert!(res.is_ok());

    let socketaddrv6_b = res.unwrap();
    assert_eq!(socketaddrv6_a, socketaddrv6_b);
}

#[test]
fn test_address_default() {
    let address = Address::default();

    let addr_str = address.to_string();
    assert_eq!(&addr_str, DEFAULT_SOCKET);

    let socket = address.to_socket().unwrap();
    let default_socket = SocketAddr::from_str(DEFAULT_SOCKET).unwrap();
    assert_eq!(socket, default_socket);
}