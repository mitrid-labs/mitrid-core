use mitrid_core::base::Result;
use mitrid_core::base::EvalMut;

use fixture::io::store::*;

#[derive(Clone)]
pub struct EvaluatorMut {}

impl EvalMut<Store, EvalMutParams, EvalMutResult> for EvaluatorMut {
    fn eval_mut(&mut self, store: &mut Store, params: &EvalMutParams) -> Result<EvalMutResult> {
        match params {
            &EvalMutParams::Clear => {
                store.clear();

                let res = EvalMutResult::Cleared;
                Ok(res)
            },
        }
    }
}