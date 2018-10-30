use mitrid_core::base::Result;
use mitrid_core::base::Datable;
use mitrid_core::io::network::ClientTransport as BasicClientTransport;
use mitrid_core::io::network::ServerTransport as BasicServerTransport;

use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::mem;

pub const BUFFER_SIZE: usize = 2048;

use fixtures::io::Address;

pub struct ClientTransport(TcpStream);

impl Clone for ClientTransport {
    fn clone(&self) -> ClientTransport {
        ClientTransport(self.0.try_clone().unwrap())
    }
}

impl BasicClientTransport<Address> for ClientTransport {
    fn connect<P: Datable>(_params: &P, addresses: &Vec<Address>) -> Result<Self> {
        if addresses.len() != 1 {
            return Err(String::from("invalid length"));
        }

        let addr = addresses[0].to_owned();

        let tcp_stream = TcpStream::connect(&addr.to_string())
                            .map_err(|e| format!("{:?}", e))?;

        let ct = ClientTransport(tcp_stream);

        Ok(ct)
    }

    fn disconnect<P: Datable>(&mut self, _params: &P) -> Result<()> {
        self.0.shutdown(Shutdown::Both)
            .map_err(|e| format!("{:?}", e)).into()
    }

    fn send<P: Datable>(&mut self, _params: &P, data: &[u8]) -> Result<()> {
        if data.len() > BUFFER_SIZE -4 {
            return Err(format!("invalid length"));
        }

        let mut msg = [0u8; BUFFER_SIZE];

        let len_msg: [u8; 4] = unsafe { mem::transmute(data.len() as u32) };

        for i in 0..4 {
            msg[i] = len_msg[i];
        }

        for i in 0..data.len() {
            msg[i+4] = data[i];
        }

        self.0.write(&msg[..])
            .map(|_| ())
            .map_err(|e| format!("{:?}", e))
    }

    fn recv<P: Datable>(&mut self, _params: &P) -> Result<Vec<Vec<u8>>> {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

        self.0.read(&mut buffer[..])
            .map_err(|e| format!("{:?}", e))?;

        let mut len_msg = [0u8; 4];

        for i in 0..4 {
            len_msg[i] = buffer[i];
        }

        let len: u32 = unsafe { mem::transmute(len_msg) };

        let mut msg = Vec::new();

        for i in 0..len as usize {
            msg.push(buffer[i+4]);
        }

        Ok(vec![msg])
    }
}

pub struct ServerTransport(TcpListener);

impl ServerTransport {
    pub fn serve_ping(addresses: &Vec<Address>) {
        let mut server = ServerTransport::listen(&(), addresses).unwrap();

        let (mut client, _) = server.accept(&()).unwrap();
        
        let recvd = client.recv(&()).unwrap();

        let msg = &recvd[0];

        client.send(&(), msg.as_slice()).unwrap();
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

        let st = ServerTransport(listener);

        Ok(st)
    }

    fn accept<P: Datable>(&mut self, _params: &P) -> Result<(ClientTransport, Vec<Address>)> {
        let (tcp_stream, socket) = self.0.accept()
                                        .map_err(|e| format!("{:?}", e))?;

        let transport = ClientTransport(tcp_stream);
        let address = Address::from_socket(&socket);

        Ok((transport, vec![address]))
    }

    fn close<P: Datable>(&mut self, _params: &P) -> Result<()> {
        Ok(())
    }
}