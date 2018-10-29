/*
use futures::Future as BasicFuture;
use futures::Stream as BasicStream;
use mitrid_core::io::ClientTransport as BasicClientTransport;

use std::thread;

use fixtures::io::Address;
use fixtures::io::{ClientTransport, ServerTransport};
*/

#[test]
fn test_ping_server() {
    /*
    thread::spawn(move || {
        let addresses = vec![Address::default()];
        let res = ServerTransport::run(&addresses);
        println!("server res: {:?}", res.clone());
        assert!(res.is_ok());
        res.unwrap();
    });

    let addresses = vec![Address::default()];
    let res = ClientTransport::connect(&(), &addresses).wait();
    assert!(res.is_ok());

    let mut client = res.unwrap();
    let msg_a = b"ping";
    let res = client.send(&(), msg_a).wait();
    println!("client res: {:?}", res.clone());
    assert!(res.is_ok());
    res.unwrap();

    let res = client.recv(&()).into_future().wait();
    assert!(res.is_ok());

    match res {
        Err(_) => {},
        Ok((msg_b, _)) => {
            println!("here we are");
            if let Some(msg_b) = msg_b {
                assert_eq!(&msg_a, &msg_b.as_slice());
                let res = client.disconnect(&()).wait();
                assert!(res.is_ok());
            } else {
                panic!("no response message")
            }
        },
    }
    */
}