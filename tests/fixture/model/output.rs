use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::{Eval, EvalMut};
use mitrid_core::model::Output as BaseOutput;

use fixture::base::Payload;
use fixture::base::eval::*;
use fixture::crypto::Digest;
use fixture::crypto::PublicKey;
use fixture::model::Amount;

pub type Output = BaseOutput<Digest, PublicKey, Amount, Payload>;

#[derive(Clone)]
pub struct OutputEvaluator {}

impl Eval<Output, PayloadEvalParams, PayloadEvalResult> for OutputEvaluator {
    fn eval(&self, output: &Output, params: &PayloadEvalParams) -> Result<PayloadEvalResult> {
        output.check()?;
        params.check()?;

        let s = output.payload.to_string();

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

impl EvalMut<Output, PayloadEvalMutParams, PayloadEvalMutResult> for OutputEvaluator {
    fn eval_mut(&mut self, output: &mut Output, params: &PayloadEvalMutParams) -> Result<PayloadEvalMutResult> {
        output.check()?;
        params.check()?;

        let s = output.payload.to_string();

        match params {
            &PayloadEvalMutParams::ToUppercase => {
                let new_s = s.to_uppercase();
                output.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToUppercase(new_s);
                Ok(res)
            },
            &PayloadEvalMutParams::ToLowercase => {
                let new_s = s.to_lowercase();
                output.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToLowercase(new_s);
                Ok(res)
            },
        }
    }
}