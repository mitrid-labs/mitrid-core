//! # Client
//!
//! `client` is the module providing the trait implemented by network clients.

use futures::Future as BasicFuture;

use base::Result;
use base::Future;
use base::numerical::Numerical;
use base::size::{ConstantSize, VariableSize};
use base::Checkable;
use base::Datable;
use base::Serializable;
use io::network::transport::Transport;
use io::network::message::Request;
use io::network::message::Response;
use io::network::client::OnError;

/// Trait implemented by network clients.
pub trait Client<T, S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   T: Transport<Ad>,
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
            C: Datable + Serializable
{
    /// Builds a list to `Request` messages to send in sequence.
    fn build<P: Datable>(&self, params: &P, addresses: &Vec<Ad>)
        -> Result<Vec<Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>>;

    /// Client behaviour when `OnError` is set to Ignore.
    fn send_ignore_on_error<P: Datable>(&self,
                                        transport: &mut T,
                                        send_params: &P,
                                        requests: &Vec<Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>,
                                        responses: &mut Vec<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>)
        -> Result<()>
    {
        send_params.check()?;
        requests.check()?;

        let mut ress = Vec::new();

        for ref req in requests {
            let ser_req = req.to_bytes()?;
            let ser_ress: Vec<Vec<u8>> = transport.send(send_params, &ser_req).wait()?;

            for ser_res in ser_ress {
                let res = Response::from_bytes(&ser_res)?;
                res.check()?;
                ress.push(res);
            }
        }

        responses.extend(ress);

        Ok(())
    }

    /// Client behaviour when `OnError` is set to Fail.
    fn send_fail_on_error<P: Datable>(&self,
                                      transport: &mut T,
                                      send_params: &P,
                                      requests: &Vec<Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>,
                                      responses: &mut Vec<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>)
        -> Result<()>
    {
        send_params.check()?;
        requests.check()?;

        let mut ress = Vec::new();

        for ref req in requests {
            let ser_req = req.to_bytes()?;
            let ser_ress = transport.send(send_params, &ser_req).wait()?;

            for ser_res in ser_ress {
                let res = Response::from_bytes(&ser_res)?;
                res.check()?;
                if res.is_error()? {
                    return Err(String::from("error response"));
                }
                ress.push(res);
            }
        }

        responses.extend(ress);

        Ok(())
    }

    /// Client behaviour when `OnError` is set to RetryAndIgnore.
    fn send_retry_and_ignore<P: Datable>(&self,
                                         transport: &mut T,
                                         send_params: &P,
                                         times: u64,
                                         requests: &Vec<Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>,
                                         responses: &mut Vec<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>)
        -> Result<()>
    {
        send_params.check()?;
        requests.check()?;
        
        let mut t = times;
        let mut step = 0; // the step in the sequence we are at
        let mut ress = Vec::new();

        while t != 0 {
            for (idx, ref req) in requests.iter().enumerate() {
                if idx < step {
                    continue;
                }

                let ser_req = req.to_bytes()?;
                let ser_ress = transport.send(send_params, &ser_req).wait()?;

                for ser_res in ser_ress {
                    let res = Response::from_bytes(&ser_res)?;
                    res.check()?;
                    
                    if res.is_error()? {
                        if t == 1 {
                            ress.push(res);
                        } else {
                            t -= 1;
                            break;
                        }
                    } else {
                        ress.push(res);
                        step += 1;
                    }
                }
            }
        }
        
        responses.extend(ress);

        Ok(())
    }

    /// Client behaviour when `OnError` is set to RetryAndFail.
    fn send_retry_and_fail<P: Datable>(&self,
                                       transport: &mut T,
                                       send_params: &P,
                                       times: u64,
                                       requests: &Vec<Request<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>,
                                       responses: &mut Vec<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>)
        -> Result<()>
    {
        send_params.check()?;
        requests.check()?;
        
        let mut t = times;
        let mut step = 0; // the step in the sequence we are at
        let mut ress = Vec::new();

        while t != 0 {
            for (idx, ref req) in requests.iter().enumerate() {
                if idx < step {
                    continue;
                }

                let ser_req = req.to_bytes()?;
                let ser_ress = transport.send(send_params, &ser_req).wait()?;

                for ser_res in ser_ress {
                    let res = Response::from_bytes(&ser_res)?;
                    res.check()?;
                    
                    if res.is_error()? {
                        if t == 1 {
                            if step != requests.len() -1 {
                                return Err(String::from("error response"));
                            }
                        } else {
                            t -= 1;
                            break;
                        }
                    } else {
                        ress.push(res);
                        step += 1;
                    }
                }
            }
        }
        
        responses.extend(ress);

        Ok(())
    }

    /// Sends a sequence of `Request`s to one or more addresses. `Request`s are build
    /// by some params and the list of addresses.
    fn send<P, CP, SP, DP>(&self,
                           params: &P,
                           addresses: &Vec<Ad>,
                           transport: &mut T,
                           connect_params: &CP,
                           send_params: &SP,
                           disconnect_params: &DP,
                           on_error: OnError)
        -> Future<Vec<Response<S, RS, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>>>
        where   P: Datable,
                CP: Datable,
                SP: Datable,
                DP: Datable
    {
        match params.check() {
            Ok(_) => {},
            Err(e) => {
                return Future::from_result(Err(e));
            }
        }

        let reqs_res = self.build(params, addresses);
        match reqs_res {
            Ok(_) => {},
            Err(e) => {
                return Future::from_result(Err(e));
            }
        }

        let requests = reqs_res.unwrap();

        let conn_res = transport.connect(connect_params, &addresses);
        match conn_res.wait() {
            Ok(_) => {},
            Err(e) => {
                return Future::from_result(Err(e));
            }
        }

        let mut responses = Vec::new();

        match on_error {
            OnError::Ignore => {
                let res = self.send_ignore_on_error(transport, send_params, &requests, &mut responses);
                match res {
                    Ok(_) => {},
                    Err(e) => {
                        return Future::from_result(Err(e));
                    }
                }
            },
            OnError::Fail => {
                let res = self.send_ignore_on_error(transport, send_params, &requests, &mut responses);
                match res {
                    Ok(_) => {},
                    Err(e) => {
                        return Future::from_result(Err(e));
                    }
                }
            },
            OnError::RetryAndIgnore(times) => {
                let res = self.send_retry_and_ignore(transport, send_params, times, &requests, &mut responses);
                match res {
                    Ok(_) => {},
                    Err(e) => {
                        return Future::from_result(Err(e));
                    }
                }
            },
            OnError::RetryAndFail(times) => {
                let res = self.send_retry_and_fail(transport, send_params, times, &requests, &mut responses);
                match res {
                    Ok(_) => {},
                    Err(e) => {
                        return Future::from_result(Err(e));
                    }
                }
            },
        }

        let disconn_res = transport.disconnect(disconnect_params, &addresses);
        match disconn_res.wait() {
            Ok(_) => {},
            Err(e) => {
                return Future::from_result(Err(e));
            }
        }

        Future::from_result(Ok(responses))
    }
}