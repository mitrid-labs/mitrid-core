use mitrid_core::base::Future;
use mitrid_core::base::Stream;
use mitrid_core::base::Datable;
use mitrid_core::io::network::ClientTransport as BasicClientTransport;
use mitrid_core::io::network::ServerTransport as BasicServerTransport;

use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};

use fixtures::io::Address;

#[derive(Clone)]
pub struct ClientTransport(Arc<Mutex<TcpStream>>);

impl BasicClientTransport<Address> for ClientTransport {
    fn connect<P: Datable>(_params: &P, addresses: &Vec<Address>) -> Future<Self> {
        if addresses.len() != 1 {
            return Future::from_result(Err(String::from("invalid length")));
        }

        let addr = addresses[0].to_owned();

        match TcpStream::connect(&addr.to_string()) {
            Err(e) => Future::from_result(Err(format!("{:?}", e))),
            Ok(tcp_stream) => {
                let ct = ClientTransport(Arc::new(Mutex::new(tcp_stream)));
                Future::from_result(Ok(ct))
            },
        }
    }

    fn disconnect<P: Datable>(&mut self, _params: &P, _addresses: &Vec<Address>) -> Future<()> {
        (*self.0.lock().unwrap()).shutdown(Shutdown::Both)
            .map_err(|e| format!("{:?}", e)).into()
    }

    fn send<P: Datable>(&mut self, _params: &P, data: &[u8]) -> Future<()> {
        (*self.0.lock().unwrap())
            .write(data)
            .map(|_| ())
            .map_err(|e| format!("{:?}", e))
            .into()
    }

    fn recv<P: Datable>(&mut self, _params: &P) -> Stream<Vec<u8>> {
        let mut buffer = Vec::new();

        let res = (*self.0.lock().unwrap()).read_to_end(&mut buffer);
        match res {
            Ok(_) => {},
            Err(e) => {
                return Stream::from_result(Err(format!("{:?}", e)));
            },
        }

        Stream::from_result(Ok(buffer))
    }
}

#[derive(Clone)]
pub struct ServerTransport(Arc<Mutex<TcpListener>>);

impl BasicServerTransport<Address, ClientTransport> for ServerTransport {
    fn listen<P: Datable>(_params: &P, addresses: &Vec<Address>) -> Future<ServerTransport> {
        if addresses.len() != 1 {
            return Future::from_result(Err(String::from("invalid length")));
        }

        let addr = addresses[0].to_owned();

        match TcpListener::bind(&addr.to_string()) {
            Err(e) => Future::from_result(Err(format!("{:?}", e))),
            Ok(listener) => {
                match listener.set_nonblocking(true) {
                    Err(e) => Future::from_result(Err(format!("{:?}", e))),
                    Ok(_) => {
                        let st = ServerTransport(Arc::new(Mutex::new(listener)));
                        Future::from_result(Ok(st))
                    },
                }
            },
        }
    }

    fn accept<P: Datable>(&mut self, _params: &P) -> Future<(ClientTransport, Vec<Address>)> {
        match (*self.0.lock().unwrap()).accept() {
            Err(e) => {
                Future::from_result(Err(format!("{:?}", e)))
            },
            Ok((tcp_stream, socket)) => {
                let transport = ClientTransport(Arc::new(Mutex::new(tcp_stream)));
                let address = Address::from_socket(&socket);

                Future::from_result(Ok((transport, vec![address])))
            },
        }
    }

    fn close<P: Datable>(&mut self, _params: &P) -> Future<()> {
        Future::from_result(Ok(()))
    }
}