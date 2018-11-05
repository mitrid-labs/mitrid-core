use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::{Eval, EvalMut};
use mitrid_core::model::Block as BaseBlock;

use fixture::base::Payload;
use fixture::base::eval::*;
use fixture::crypto::Digest;
use fixture::crypto::{PublicKey, Signature};
use fixture::crypto::Proof;
use fixture::model::Amount;

pub type Block = BaseBlock<Digest, Amount, Payload, PublicKey, Signature, Payload, Payload, Payload, Proof>;

#[derive(Clone)]
pub struct BlockEvaluator {}

impl Eval<Block, PayloadEvalParams, PayloadEvalResult> for BlockEvaluator {
    fn eval(&self, block: &Block, params: &PayloadEvalParams) -> Result<PayloadEvalResult> {
        block.check()?;
        params.check()?;

        let s = block.payload.to_string();

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

impl EvalMut<Block, PayloadEvalMutParams, PayloadEvalMutResult> for BlockEvaluator {
    fn eval_mut(&mut self, block: &mut Block, params: &PayloadEvalMutParams) -> Result<PayloadEvalMutResult> {
        block.check()?;
        params.check()?;

        let s = block.payload.to_string();

        match params {
            &PayloadEvalMutParams::ToUppercase => {
                let new_s = s.to_uppercase();
                block.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToUppercase(new_s);
                Ok(res)
            },
            &PayloadEvalMutParams::ToLowercase => {
                let new_s = s.to_lowercase();
                block.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToLowercase(new_s);
                Ok(res)
            },
        }
    }
}