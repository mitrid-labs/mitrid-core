use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::{Eval, EvalMut};
use mitrid_core::model::BlockGraph as BaseBlockGraph;

use fixture::base::Payload;
use fixture::base::eval::*;
use fixture::crypto::Digest;

pub type BlockGraph = BaseBlockGraph<Digest, Payload>;

#[derive(Clone)]
pub struct BlockGraphEvaluator {}

impl Eval<BlockGraph, PayloadEvalParams, PayloadEvalResult> for BlockGraphEvaluator {
    fn eval(&self, blockgraph: &BlockGraph, params: &PayloadEvalParams) -> Result<PayloadEvalResult> {
        blockgraph.check()?;
        params.check()?;

        let s = blockgraph.payload.to_string();

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

impl EvalMut<BlockGraph, PayloadEvalMutParams, PayloadEvalMutResult> for BlockGraphEvaluator {
    fn eval_mut(&mut self, blockgraph: &mut BlockGraph, params: &PayloadEvalMutParams) -> Result<PayloadEvalMutResult> {
        blockgraph.check()?;
        params.check()?;

        let s = blockgraph.payload.to_string();

        match params {
            &PayloadEvalMutParams::ToUppercase => {
                let new_s = s.to_uppercase();
                blockgraph.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToUppercase(new_s);
                Ok(res)
            },
            &PayloadEvalMutParams::ToLowercase => {
                let new_s = s.to_lowercase();
                blockgraph.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToLowercase(new_s);
                Ok(res)
            },
        }
    }
}