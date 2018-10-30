use mitrid_core::io::ClientTransport as BasicClientTransport;

use std::thread;
use std::time::Duration;

use fixtures::io::Address;
use fixtures::io::{ClientTransport, ServerTransport};

#[test]
fn test_ping_server() {
    thread::spawn(move || {
        let addresses = vec![Address::default()];
        ServerTransport::serve_ping(&addresses);
    });

    thread::sleep(Duration::from_millis(100));

    let addresses = vec![Address::default()];
    let res = ClientTransport::connect(&(), &addresses);
    assert!(res.is_ok());

    let mut client = res.unwrap();
    let msg_a = b"ping";
    let res = client.send(&(), msg_a);
    assert!(res.is_ok());

    let res = client.recv(&());
    assert!(res.is_ok());

    let responses = res.unwrap();
    assert!(responses.len() == 1);

    let msg_b = responses[0].as_slice();

    assert_eq!(msg_a, msg_b);

    let res = client.disconnect(&());
    assert!(res.is_ok())
}