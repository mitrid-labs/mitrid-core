use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::{Eval, EvalMut};
use mitrid_core::model::Transaction as BaseTransaction;

use fixture::base::Payload;
use fixture::base::eval::*;
use fixture::crypto::Digest;
use fixture::crypto::{PublicKey, Signature};
use fixture::model::Amount;

pub type Transaction = BaseTransaction<Digest, Amount, Payload, PublicKey, Signature, Payload, Payload>;


#[derive(Clone)]
pub struct TransactionEvaluator {}

impl Eval<Transaction, PayloadEvalParams, PayloadEvalResult> for TransactionEvaluator {
    fn eval(&self, transaction: &Transaction, params: &PayloadEvalParams) -> Result<PayloadEvalResult> {
        transaction.check()?;
        params.check()?;

        let s = transaction.payload.to_string();

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

impl EvalMut<Transaction, PayloadEvalMutParams, PayloadEvalMutResult> for TransactionEvaluator {
    fn eval_mut(&mut self, transaction: &mut Transaction, params: &PayloadEvalMutParams) -> Result<PayloadEvalMutResult> {
        transaction.check()?;
        params.check()?;

        let s = transaction.payload.to_string();

        match params {
            &PayloadEvalMutParams::ToUppercase => {
                let new_s = s.to_uppercase();
                transaction.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToUppercase(new_s);
                Ok(res)
            },
            &PayloadEvalMutParams::ToLowercase => {
                let new_s = s.to_lowercase();
                transaction.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToLowercase(new_s);
                Ok(res)
            },
        }
    }
}