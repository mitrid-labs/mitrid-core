use mitrid_core::io::ClientTransport as BasicClientTransport;

use std::thread;
use std::time::Duration;

use fixture::io::Address;
use fixture::io::{ClientTransport, ServerTransport};

#[test]
fn test_ping_server() {
    thread::spawn(move || {
        let address = Address::default();
        ServerTransport::serve_ping(&address);
    });

    thread::sleep(Duration::from_millis(100));

    let address = Address::default();
    let res = ClientTransport::connect(&address);
    assert!(res.is_ok());

    let mut client = res.unwrap();
    let msg_a = b"ping";
    let res = client.send(msg_a);
    assert!(res.is_ok());

    let res = client.recv();
    assert!(res.is_ok());

    let response = res.unwrap();
    let msg_b = response.as_slice();

    assert_eq!(msg_a, msg_b);

    let res = client.disconnect();
    assert!(res.is_ok())
}