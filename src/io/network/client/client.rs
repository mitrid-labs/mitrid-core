//! # Client
//!
//! `client` is the module providing the trait implemented by network clients.

use base::Result;
use base::size::{ConstantSize, VariableSize};
use base::Checkable;
use base::Datable;
use base::Serializable;
use io::network::transport::ClientTransport;
use io::network::message::Request;
use io::network::message::Response;
use io::network::client::OnError;

/// Trait implemented by network clients.
pub trait Client<CT, Ad, S, D, MP>
    where   CT: ClientTransport<Ad>,
            Ad: Ord + Datable + VariableSize + Serializable,
            S: Datable + Serializable,
            D: Ord + Datable + ConstantSize + Serializable,
            MP: Datable + Serializable,
            Self: 'static + Sized + Clone + Send + Sync
{
    /// Client behaviour when `OnError` is set to Ignore.
    fn send_ignore_on_error(&mut self,
                            transport: &mut CT,
                            requests: &Vec<Request<S, D, MP>>,
                            responses: &mut Vec<Response<S, D, MP>>)
        -> Result<()>
    {
        requests.check()?;

        let mut ress = Vec::new();

        for ref req in requests {
            let ser_req = req.to_bytes()?;
            transport.send(&ser_req)?;

            let ser_res = transport.recv()?;
            let res = Response::from_bytes(&ser_res)?;
            res.check()?;
            ress.push(res);
        }

        responses.extend(ress);

        Ok(())
    }

    /// Client behaviour when `OnError` is set to Fail.
    fn send_fail_on_error(&mut self,
                          transport: &mut CT,
                          requests: &Vec<Request<S, D, MP>>,
                          responses: &mut Vec<Response<S, D, MP>>)
        -> Result<()>
    {
        requests.check()?;

        let mut ress = Vec::new();

        for ref req in requests {
            let ser_req = req.to_bytes()?;
            transport.send(&ser_req)?;

            let ser_res = transport.recv()?;
            let res = Response::from_bytes(&ser_res)?;
            res.check()?;
            
            if res.message.is_error() {
                return Err(String::from("error response"));
            }

            ress.push(res);
        }

        responses.extend(ress);

        Ok(())
    }

    /// Client behaviour when `OnError` is set to RetryAndIgnore.
    fn send_retry_and_ignore(&mut self,
                             transport: &mut CT,
                             times: u64,
                             requests: &Vec<Request<S, D, MP>>,
                             responses: &mut Vec<Response<S, D, MP>>)
        -> Result<()>
    {
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
                transport.send(&ser_req)?;

                let ser_res = transport.recv()?;
                let res = Response::from_bytes(&ser_res)?;
                res.check()?;
                    
                if res.message.is_error() {
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
        
        responses.extend(ress);

        Ok(())
    }

    /// Client behaviour when `OnError` is set to RetryAndFail.
    fn send_retry_and_fail(&mut self,
                           transport: &mut CT,
                           times: u64,
                           requests: &Vec<Request<S, D, MP>>,
                           responses: &mut Vec<Response<S, D, MP>>)
        -> Result<()>
    {
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
                transport.send(&ser_req)?;
                
                let ser_res = transport.recv()?;
                let res = Response::from_bytes(&ser_res)?;
                res.check()?;
                
                if res.message.is_error() {
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
        
        responses.extend(ress);

        Ok(())
    }

    /// Sends a sequence of `Request`s to one or more addresses. `Request`s are build
    /// by some params and the list of addresses.
    fn send(&mut self,
            address: &Ad,
            requests: &Vec<Request<S, D, MP>>,
            on_error: OnError)
        -> Result<Vec<Response<S, D, MP>>>
    {
        address.check()?;
        address.check_size()?;

        requests.check()?;
        
        on_error.check()?;

        let mut transport = CT::connect(&address)?;

        let mut responses = Vec::new();

        match on_error {
            OnError::Ignore => {
                self.send_ignore_on_error(&mut transport, &requests, &mut responses)?;
            },
            OnError::Fail => {
                self.send_fail_on_error(&mut transport, &requests, &mut responses)?;
            },
            OnError::RetryAndIgnore(times) => {
                self.send_retry_and_ignore(&mut transport, times, &requests, &mut responses)?;
            },
            OnError::RetryAndFail(times) => {
                self.send_retry_and_fail(&mut transport, times, &requests, &mut responses)?;
            },
        }

        transport.disconnect()?;

        Ok(responses)
    }
}