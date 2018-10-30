use mitrid_core::base::Result;
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
    fn connect<P: Datable>(_params: &P, addresses: &Vec<Address>) -> Result<Self> {
        if addresses.len() != 1 {
            return Err(String::from("invalid length"));
        }

        let addr = addresses[0].to_owned();

        let tcp_stream = TcpStream::connect(&addr.to_string())
                            .map_err(|e| format!("{:?}", e))?;

        let ct = ClientTransport(Arc::new(Mutex::new(tcp_stream)));

        Ok(ct)
    }

    fn disconnect<P: Datable>(&mut self, _params: &P) -> Result<()> {
        (*self.0.lock().unwrap()).shutdown(Shutdown::Both)
            .map_err(|e| format!("{:?}", e)).into()
    }

    fn send<P: Datable>(&mut self, _params: &P, data: &[u8]) -> Result<()> {
        (*self.0.lock().unwrap())
            .write(data)
            .map(|_| ())
            .map_err(|e| format!("{:?}", e))
    }

    fn recv<P: Datable>(&mut self, _params: &P) -> Result<Vec<Vec<u8>>> {
        let mut buffer = Vec::new();

        (*self.0.lock().unwrap())
            .read_to_end(&mut buffer)
            .map_err(|e| format!("{:?}", e))?;

        Ok(vec![buffer])
    }
}

#[derive(Clone)]
pub struct ServerTransport(Arc<Mutex<TcpListener>>);

impl ServerTransport {
    pub fn run(addresses: &Vec<Address>) -> Result<()> {
        let mut server = ServerTransport::listen(&(), addresses)?;

        let (mut client, _) = server.accept(&())?;
        
        for recvd in client.recv(&())? {
            client.send(&(), &recvd)?
        }

        Ok(())
    }
}

impl BasicServerTransport<Address, ClientTransport> for ServerTransport {
    fn listen<P: Datable>(_params: &P, addresses: &Vec<Address>) -> Result<ServerTransport> {
        if addresses.len() != 1 {
            return Err(String::from("invalid length"));
        }

        let addr = addresses[0].to_owned();

        let listener = TcpListener::bind(&addr.to_string())
                            .map_err(|e| format!("{:?}", e))?;

        let st = ServerTransport(Arc::new(Mutex::new(listener)));

        Ok(st)
    }

    fn accept<P: Datable>(&mut self, _params: &P) -> Result<(ClientTransport, Vec<Address>)> {
        let (tcp_stream, socket) = (*self.0.lock().unwrap())
                                        .accept()
                                        .map_err(|e| format!("{:?}", e))?;

        let transport = ClientTransport(Arc::new(Mutex::new(tcp_stream)));
        let address = Address::from_socket(&socket);

        Ok((transport, vec![address]))
    }

    fn close<P: Datable>(&mut self, _params: &P) -> Result<()> {
        Ok(())
    }
}