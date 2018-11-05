use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::{Eval, EvalMut};
use mitrid_core::io::Message as BasicMessage;

use fixture::base::Payload;
use fixture::base::eval::*;
use fixture::crypto::Digest;
use fixture::io::Address;

pub type Message = BasicMessage<(), Address, Payload, Digest, Payload>;

#[derive(Clone)]
pub struct MessageEvaluator {}

impl Eval<Message, PayloadEvalParams, PayloadEvalResult> for MessageEvaluator {
    fn eval(&self, message: &Message, params: &PayloadEvalParams) -> Result<PayloadEvalResult> {
        message.check()?;
        params.check()?;

        let s = message.payload.to_string();

        match params {
            &PayloadEvalParams::Const => {
                let res = PayloadEvalResult::Const(s);
                Ok(res)
            },
            &PayloadEvalParams::IsEmpty => {
                let res = PayloadEvalResult::IsEmpty(s.is_empty());
                Ok(res)
            },
        }
    }
}

impl EvalMut<Message, PayloadEvalMutParams, PayloadEvalMutResult> for MessageEvaluator {
    fn eval_mut(&mut self, message: &mut Message, params: &PayloadEvalMutParams) -> Result<PayloadEvalMutResult> {
        message.check()?;
        params.check()?;

        let s = message.payload.to_string();

        match params {
            &PayloadEvalMutParams::ToUppercase => {
                let new_s = s.to_uppercase();
                message.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToUppercase(new_s);
                Ok(res)
            },
            &PayloadEvalMutParams::ToLowercase => {
                let new_s = s.to_lowercase();
                message.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToLowercase(new_s);
                Ok(res)
            },
        }
    }
}