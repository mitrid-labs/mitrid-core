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
use base::{Eval, EvalMut};
use io::store::Store;
use io::network::transport::{ClientTransport, ServerTransport};
use io::network::server::Handler;
use io::network::server::Router;
use io::network::message::{Request, Response};

/// Trait implemented by network servers.
pub trait Server<St, StS, ST, CT, H, R, S, Ad, NP, D, MP>
    where   St: Store<StS>,
            StS: Datable + Serializable,
            ST: ServerTransport<Ad, CT>,
            CT: ClientTransport<Ad>,
            H: Handler<St, StS, S, Ad, NP, D, MP>,
            R: Router<St, StS, S, Ad, NP, D, MP, H>,
            S: Datable + Serializable,
            Ad: Ord + Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            MP: Datable + Serializable
{
    /// Serves incoming requests.
    fn serve<Ev, EvM>(store: St,
                      handler: H,
                      router: R,
                      addresses: &Vec<Ad>,
                      thread_limit: u64,
                      evaluator: Ev,
                      evaluator_mut: EvM)
                -> Result<()>
        where   Ev: 'static + Send + Sync + Eval<St, Request<S, Ad, NP, D, MP>, Response<S, Ad, NP, D, MP>>,
                EvM: 'static + Send + EvalMut<St, Request<S, Ad, NP, D, MP>, Response<S, Ad, NP, D, MP>>
    {
        addresses.check()?;
        for ref address in addresses {
            address.check_size()?;
        }

        let mut transport = ST::listen(addresses)?;

        let threads_num = Arc::new(Mutex::new(0));
        let store = Arc::new(Mutex::new(store));
        let handler = Arc::new(Mutex::new(handler));
        let router = Arc::new(Mutex::new(router));
        let evaluator = Arc::new(evaluator);
        let evaluator_mut = Arc::new(Mutex::new(evaluator_mut));

        loop {
            while *threads_num.lock().unwrap() < thread_limit {

                transport.accept()
                    .and_then(|(mut transport, _)| {

                        for ser_req in transport.recv()? {
                            let req = Request::from_bytes(ser_req.as_slice())?;
                            let mut transport = transport.clone();
                            let store = store.clone();
                            let handler = handler.clone(); 
                            let router = router.clone();
                            let evaluator = evaluator.clone();
                            let evaluator_mut = evaluator_mut.clone();
                            let threads_num = threads_num.clone();
                            
                            let _ = thread::spawn(move || {
                                *threads_num.lock().unwrap() += 1;

                                let router = &mut *router.lock().unwrap();
                                let store = &mut *store.lock().unwrap();
                                let handler = &mut *handler.lock().unwrap();
                                let evaluator: &Ev = &*evaluator;
                                let evaluator_mut: &mut EvM = &mut *evaluator_mut.lock().unwrap();

                                router.route(store, handler, &req, &*evaluator, evaluator_mut)
                                    .and_then(|res| {
                                        transport.send(&res.to_bytes().unwrap())
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