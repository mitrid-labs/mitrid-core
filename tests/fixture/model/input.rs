use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::{Eval, EvalMut};
use mitrid_core::model::Input as BaseInput;

use fixture::base::Payload;
use fixture::base::eval::*;
use fixture::crypto::Digest;
use fixture::crypto::{PublicKey, Signature};
use fixture::model::Amount;

pub type Input = BaseInput<Digest, Amount, Payload, PublicKey, Signature>;

#[derive(Clone)]
pub struct InputEvaluator {}

impl Eval<Input, PayloadEvalParams, PayloadEvalResult> for InputEvaluator {
    fn eval(&self, input: &Input, params: &PayloadEvalParams) -> Result<PayloadEvalResult> {
        input.check()?;
        params.check()?;

        let s = input.payload.to_string();

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

impl EvalMut<Input, PayloadEvalMutParams, PayloadEvalMutResult> for InputEvaluator {
    fn eval_mut(&mut self, input: &mut Input, params: &PayloadEvalMutParams) -> Result<PayloadEvalMutResult> {
        input.check()?;
        params.check()?;

        let s = input.payload.to_string();

        match params {
            &PayloadEvalMutParams::ToUppercase => {
                let new_s = s.to_uppercase();
                input.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToUppercase(new_s);
                Ok(res)
            },
            &PayloadEvalMutParams::ToLowercase => {
                let new_s = s.to_lowercase();
                input.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToLowercase(new_s);
                Ok(res)
            },
        }
    }
}