//! # Server
//!
//! `server` is the module providing the trait implemented by network servers.

use std::thread;
use std::sync::{Arc, Mutex};

use base::Result;
use base::size::{ConstantSize, VariableSize};
use base::Checkable;
use base::Serializable;
use base::Datable;
use io::store::Store;
use io::network::transport::{ClientTransport, ServerTransport};
use io::network::server::Handler;
use io::network::server::Router;
use io::network::message::Request;

/// Trait implemented by network servers.
pub trait Server<St, StS, StK, StV, StP, StPC, StRC, ST, CT, H, R, S, Ad, NP, D, MP>
    where   St: Store<StS, StP, StPC, StRC>,
            StS: Datable + Serializable,
            StK: Ord + Datable + Serializable,
            StV: Datable + Serializable,
            StP: Datable,
            StPC: Datable + Serializable,
            StRC: Datable + Serializable,
            ST: ServerTransport<Ad, CT>,
            CT: ClientTransport<Ad>,
            H: Handler<St, StS, StK, StV, StP, StPC, StRC, S, Ad, NP, D, MP>,
            R: Router<St, StS, StK, StV, StP, StPC, StRC, S, Ad, NP, D, MP>,
            S: Datable + Serializable,
            Ad: Ord + Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            MP: Datable + Serializable
{
    /// Serves incoming requests.
    fn serve<P, LP, RcvP, SP, RP>(params: &P,
                                  store: St,
                                  listen_params: &LP,
                                  recv_params: &RcvP,
                                  send_params: &SP,
                                  handler: H,
                                  router: R,
                                  route_params: RP,
                                  addresses: &Vec<Ad>,
                                  thread_limit: u64)
        -> Result<()>
        where   P: Datable,
                LP: Datable,
                RcvP: Datable,
                SP: Datable,
                RP: Datable
    {
        params.check()?;

        listen_params.check()?;
        
        recv_params.check()?;
        
        send_params.check()?;
        
        addresses.check()?;

        let mut transport = ST::listen(listen_params, addresses)?;

        let threads_num = Arc::new(Mutex::new(0));
        let store = Arc::new(Mutex::new(store));
        let handler = Arc::new(handler);
        let router = Arc::new(router);

        loop {
            while *threads_num.lock().unwrap() < thread_limit {

                transport.accept(recv_params)
                    .and_then(|(mut transport, _)| {

                        for ser_req in transport.recv(recv_params)? {
                            let req = Request::from_bytes(ser_req.as_slice())?;
                            let mut transport = transport.clone();
                            let store = store.clone();
                            let send_params = send_params.clone();
                            let handler = handler.clone(); 
                            let router = router.clone();
                            let route_params = route_params.clone();
                            let threads_num = threads_num.clone();
                            
                            let _ = thread::spawn(move || {
                                *threads_num.lock().unwrap() += 1;

                                let store = &mut *store.lock().unwrap();

                                router.route(store, &*handler, &route_params, &req)
                                    .and_then(|res| {
                                        transport.send(&send_params, &res.to_bytes().unwrap())
                                    })
                                    .or_else(|e| Err(format!("{:}", e)))
                            })
                            .join()
                            .map_err(|e| format!("{:?}", e))
                            .unwrap();
                        }

                        Ok(())
                    })
                    .or_else(|e| Err(format!("{:?}", e)))?;
                    
            }

            *threads_num.lock().unwrap() = 0;
        }
    }
}