use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::io::network::ClientTransport as BasicClientTransport;
use mitrid_core::io::network::ServerTransport as BasicServerTransport;

use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::mem;

pub const BUFFER_SIZE: usize = 2048;

use fixture::io::Address;

pub struct ClientTransport(TcpStream);

impl Clone for ClientTransport {
    fn clone(&self) -> ClientTransport {
        ClientTransport(self.0.try_clone().unwrap())
    }
}

impl Checkable for ClientTransport {}

impl BasicClientTransport<Address> for ClientTransport {
    fn connect(address: &Address) -> Result<Self> {
        let tcp_stream = TcpStream::connect(&address.to_string())
                            .map_err(|e| format!("{:?}", e))?;

        let ct = ClientTransport(tcp_stream);

        Ok(ct)
    }

    fn disconnect(&mut self) -> Result<()> {
        self.0.shutdown(Shutdown::Both)
            .map_err(|e| format!("{:?}", e)).into()
    }

    fn send(&mut self, data: &[u8]) -> Result<()> {
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

    fn recv(&mut self) -> Result<Vec<Vec<u8>>> {
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
    pub fn serve_ping(address: &Address) {
        let mut server = ServerTransport::listen(address).unwrap();

        let (mut client, _) = server.accept().unwrap();
        
        let recvd = client.recv().unwrap();

        let msg = &recvd[0];

        client.send(msg.as_slice()).unwrap();
    }
}

impl Checkable for ServerTransport {}

impl BasicServerTransport<Address, ClientTransport> for ServerTransport {
    fn listen(address: &Address) -> Result<ServerTransport> {
        let listener = TcpListener::bind(&address.to_string())
                            .map_err(|e| format!("{:?}", e))?;

        let st = ServerTransport(listener);

        Ok(st)
    }

    fn accept(&mut self) -> Result<(ClientTransport, Address)> {
        let (tcp_stream, socket) = self.0.accept()
                                        .map_err(|e| format!("{:?}", e))?;

        let transport = ClientTransport(tcp_stream);
        let address = Address::from_socket(&socket);

        Ok((transport, address))
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }
}