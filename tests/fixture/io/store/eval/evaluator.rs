use mitrid_core::base::Result;
use mitrid_core::base::Sizable;
use mitrid_core::base::{Eval, EvalMut};

use fixture::io::store::*;

#[derive(Clone)]
pub struct StoreEvaluator {}

impl Eval<Store, StoreEvalParams, StoreEvalResult> for StoreEvaluator {
    fn eval(&self, store: &Store, params: &StoreEvalParams) -> Result<StoreEvalResult> {
        match params {
            &StoreEvalParams::Size => {
                let res = StoreEvalResult::new_size(store.size());
                Ok(res)
            },
            &StoreEvalParams::Dump(ref dump_params) => {
                match dump_params {
                    &DumpParams::Sessions => {
                        let sessions = &*store.sessions.lock().unwrap();

                        let mut values = Vec::new();
                        for value in sessions.values() {
                            values.push(value.to_owned());
                        }

                        let dump = DumpSessions::new(&values)?;

                        StoreEvalResult::new_dump_sessions(&dump)
                    },
                    &DumpParams::Items => {
                        let items = &*store.items.lock().unwrap();

                        let mut key_values = Vec::new();
                        for (key, value) in items.iter() {
                            key_values.push((key.to_owned(), value.to_owned()));
                        }

                        let dump = DumpItems::new(&key_values);

                        StoreEvalResult::new_dump_items(&dump)
                    },
                    &DumpParams::All => {
                        let sessions = &*store.sessions.lock().unwrap();

                        let mut session_values = Vec::new();
                        for value in sessions.values() {
                            session_values.push(value.to_owned());
                        }

                        let items = &*store.items.lock().unwrap();

                        let mut item_values = Vec::new();
                        for (key, value) in items.iter() {
                            item_values.push((key.to_owned(), value.to_owned()));
                        }
                        
                        let dump = DumpAll::new(&session_values, &item_values)?;

                        StoreEvalResult::new_dump_all(&dump)
                    },
                }
            },
        }
    }
}

impl EvalMut<Store, StoreEvalMutParams, StoreEvalMutResult> for StoreEvaluator {
    fn eval_mut(&mut self, store: &mut Store, params: &StoreEvalMutParams) -> Result<StoreEvalMutResult> {
        match params {
            &StoreEvalMutParams::Clear => {
                store.clear();

                let res = StoreEvalMutResult::Cleared;
                Ok(res)
            },
        }
    }
}