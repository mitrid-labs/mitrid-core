//! # Server
//!
//! `server` is the module providing the trait implemented by network servers.

use futures::Future as BasicFuture;

use std::thread;
use std::sync::{Arc, Mutex};

use base::Result;
use base::size::{ConstantSize, VariableSize};
use base::numerical::Numerical;
use base::Checkable;
use base::Serializable;
use base::Datable;
use io::store::Store;
use io::network::transport::Transport;
use io::network::server::Handler;
use io::network::server::Router;
use io::network::message::Request;

/// Trait implemented by network servers.
pub trait Server<St, StS, StK, StV, T, H, R, S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   St: 'static + Store<StS, StK, StV>,
            StS: Datable + Serializable,
            StK: Datable + Serializable,
            StV: Datable + Serializable,
            T: 'static + Transport<Ad>,
            H: 'static + Handler<St, StS, StK, StV, S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>,
            R: 'static + Router<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>,
            S: Datable + Serializable,
            RS: Datable + Serializable,
            Ad: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            Pr: Datable + Serializable,
            Am: Numerical + Serializable,
            IP: Datable + Serializable,
            OP: Datable + Serializable,
            TP: Datable + Serializable,
            BP: Datable + Serializable,
            BGP: Datable + Serializable,
            C: Datable + Serializable,
            Self: Clone + Sized + Sync
{
    /// Serves incoming requests.
    fn serve<P, LP, RcvP, SP, RP>(params: &P,
                                  store: St,
                                  mut transport: T,
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

        transport.listen(listen_params, addresses).wait()?;

        let threads_num = Arc::new(Mutex::new(0));
        let store = Arc::new(Mutex::new(store));
        let handler = Arc::new(handler);
        let router = Arc::new(router);

        loop {
            while *threads_num.lock().unwrap() < thread_limit {

                let res = transport.recv(recv_params)
                            .and_then(|(transport, ser_reqs)| {
                                for ser_req in ser_reqs {
                                    match Request::from_bytes(ser_req.as_slice()) {
                                        Err(e) => {
                                            return Err(e);
                                        },
                                        Ok(req) => {
                                            let store = store.clone();
                                            let mut transport = transport.clone();
                                            let send_params = send_params.clone();
                                            let handler = handler.clone(); 
                                            let router = router.clone();
                                            let route_params = route_params.clone();
                                            let threads_num = threads_num.clone();
                                            
                                            let res = thread::spawn(move || {
                                                *threads_num.lock().unwrap() += 1;

                                                let store = &mut *store.lock().unwrap();

                                                router.route(store, &*handler, &route_params, &req)
                                                    .and_then(|res| {
                                                        transport.send(&send_params, &res.to_bytes().unwrap()).wait()
                                                    })
                                                    .or_else(|e| Err(format!("{:}", e)))
                                            })
                                            .join()
                                            .map_err(|e| format!("{:?}", e));

                                            match res {
                                                Err(e) => {
                                                    return Err(e);
                                                },
                                                Ok(_) => {}
                                            }
                                        },
                                    }
                                }

                                Ok(())
                            })
                            .or_else(|e| Err(format!("{:?}", e)))
                            .wait();

                match res {
                    Ok(()) => {},
                    Err(e) => {
                        return Err(e);
                    },
                }
            }

            *threads_num.lock().unwrap() = 0;
        }
    }
}