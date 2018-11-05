use mitrid_core::base::Result;
use mitrid_core::base::Checkable;
use mitrid_core::base::{Eval, EvalMut};
use mitrid_core::model::Wallet as BaseWallet;

use fixture::base::Payload;
use fixture::base::eval::*;
use fixture::crypto::Digest;
use fixture::crypto::{SecretKey, PublicKey, Signature};

pub type Wallet = BaseWallet<Digest, SecretKey, PublicKey, Signature, Payload>;

#[derive(Clone)]
pub struct WalletEvaluator {}

impl Eval<Wallet, PayloadEvalParams, PayloadEvalResult> for WalletEvaluator {
    fn eval(&self, wallet: &Wallet, params: &PayloadEvalParams) -> Result<PayloadEvalResult> {
        wallet.check()?;
        params.check()?;

        let s = wallet.payload.to_string();

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

impl EvalMut<Wallet, PayloadEvalMutParams, PayloadEvalMutResult> for WalletEvaluator {
    fn eval_mut(&mut self, wallet: &mut Wallet, params: &PayloadEvalMutParams) -> Result<PayloadEvalMutResult> {
        wallet.check()?;
        params.check()?;

        let s = wallet.payload.to_string();

        match params {
            &PayloadEvalMutParams::ToUppercase => {
                let new_s = s.to_uppercase();
                wallet.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToUppercase(new_s);
                Ok(res)
            },
            &PayloadEvalMutParams::ToLowercase => {
                let new_s = s.to_lowercase();
                wallet.payload = Payload::new(&new_s);

                let res = PayloadEvalMutResult::ToLowercase(new_s);
                Ok(res)
            },
        }
    }
}