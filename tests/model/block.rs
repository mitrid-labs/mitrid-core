use mitrid_core::base::Checkable;
use mitrid_core::base::Sizable;
use mitrid_core::base::Serializable;
use mitrid_core::util::Version;
use mitrid_core::base::Meta;
use mitrid_core::io::Storable;

use fixture::base::eval::*;
use fixture::base::Payload;
use fixture::crypto::{Digest, Hasher};
use fixture::crypto::Prover;
use fixture::model::Amount;
use fixture::model::coin::*;
use fixture::model::input::*;
use fixture::model::output::*;
use fixture::model::transaction::*;
use fixture::model::blocknode::*;
use fixture::model::block::*;
use fixture::io::store::*;

#[test]
fn test_block_meta() {
    let valid_meta = Meta::default();
    
    let res = Block::new().meta(&valid_meta);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;

    let res = Block::new().meta(&invalid_meta);
    assert!(res.is_err())
}

#[test]
fn test_block_prev_blocks() {
    let block_height = 0;

    let mut bn = BlockNode::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .block_data(&Digest::default(), block_height)
                    .unwrap();

    let res = Block::new().prev_blocks(&vec![bn.clone(), bn.clone()]);
    assert!(res.is_err());

    let res = Block::new().prev_blocks(&vec![bn.clone()]);
    assert!(res.is_ok());

    let mut invalid_version = Version::default();
    invalid_version.buildmeta = "/\\".into();

    let mut invalid_meta = Meta::default();
    invalid_meta.version = invalid_version;
    bn.meta = invalid_meta;

    let res = Block::new()
                .prev_blocks(&vec![bn]);

    assert!(res.is_err());
}

#[test]
fn test_block_transactions() {
    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .finalize(&mut hasher)
                    .unwrap();

    let mut hasher = Hasher{};

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let mut tx = Transaction::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .inputs(&vec![input])
                    .unwrap()
                    .outputs(&vec![output])
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let res = Block::new().transactions(&vec![tx.clone(), tx.clone()]);
    assert!(res.is_err());

    let res = Block::new().transactions(&vec![tx.clone()]);
    assert!(res.is_ok());

    tx.inputs_len += 1;

    let res = Block::new()
                .transactions(&vec![tx]);
    assert!(res.is_err());
}

#[test]
fn test_block_payload() {
    let payload = Payload::default();

    let res = Block::new().payload(&payload);
    assert!(res.is_ok())
}

#[test]
fn test_block_prove() {
    let bits = 3;
    let mut prover = Prover::new(bits);

    let res = Block::new().prove(&mut prover);
    assert!(res.is_ok());
}

#[test]
fn test_block_verify_proof() {
    let bits = 3;
    let mut prover = Prover::new(bits);

    let mut block = Block::new()
                        .prove(&mut prover)
                        .unwrap();
    
    let res = block.clone().verify_proof(&mut prover);
    assert!(res.is_ok());
    assert!(res.unwrap());

    if block.proof.is_some() {
        block.proof = None;
    } else {
        block.proof = Some(0);
    }

    let res = block.verify_proof(&mut prover);
    assert!(res.is_ok());
    assert!(!res.unwrap());
}

#[test]
fn test_block_check_proof() {
    let bits = 3;
    let mut prover = Prover::new(bits);

    let mut block = Block::new()
                        .prove(&mut prover)
                        .unwrap();
    
    let res = block.clone().check_proof(&mut prover);
    assert!(res.is_ok());

    if block.proof.is_some() {
        block.proof = None;
    } else {
        block.proof = Some(0);
    }

    let res = block.check_proof(&mut prover);
    assert!(res.is_err());
}

#[test]
fn test_block_digest() {
    let block = Block::new();

    let mut hasher = Hasher{};

    let res = block.digest(&mut hasher);
    assert!(res.is_ok());
}

#[test]
fn test_block_verify_digest() {
    let mut block = Block::new();

    let mut hasher = Hasher{};

    block.id = block.digest(&mut hasher).unwrap();
    
    let res = block.verify_digest(&mut hasher);
    assert!(res.is_ok());
    assert!(res.unwrap())
}

#[test]
fn test_block_check_digest() {
    let mut block = Block::new();

    let mut hasher = Hasher{};

    block.id = block.digest(&mut hasher).unwrap();
    
    let res = block.check_digest(&mut hasher);
    assert!(res.is_ok())
}

#[test]
fn test_block_finalize() {
    let block_height = 0;

    let bn = BlockNode::new()
                .meta(&Meta::default())
                .unwrap()
                .block_data(&Digest::default(), block_height)
                .unwrap();

    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .finalize(&mut hasher)
                    .unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let tx = Transaction::new()
                .meta(&Meta::default())
                .unwrap()
                .inputs(&vec![input])
                .unwrap()
                .outputs(&vec![output])
                .unwrap()
                .payload(&Payload::default())
                .unwrap()
                .finalize(&mut hasher)
                .unwrap();

    let bits = 3;
    let mut prover = Prover::new(bits);

    let mut block = Block::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .prev_blocks(&vec![bn.clone()])
                        .unwrap()
                        .transactions(&vec![tx.clone()])
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .prove(&mut prover)
                        .unwrap();

    let res = block.clone().finalize(&mut hasher);
    assert!(res.is_ok());

    block.transactions_len += 1;

    let res = block.finalize(&mut hasher);
    assert!(res.is_err());
}

#[test]
fn test_block_check() {
    let block_height = 0;

    let bn = BlockNode::new()
                .meta(&Meta::default())
                .unwrap()
                .block_data(&Digest::default(), block_height)
                .unwrap();

    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .finalize(&mut hasher)
                    .unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let tx = Transaction::new()
                .meta(&Meta::default())
                .unwrap()
                .inputs(&vec![input])
                .unwrap()
                .outputs(&vec![output])
                .unwrap()
                .payload(&Payload::default())
                .unwrap()
                .finalize(&mut hasher)
                .unwrap();

    let bits = 3;
    let mut prover = Prover::new(bits);

    let mut block = Block::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .prev_blocks(&vec![bn.clone()])
                        .unwrap()
                        .transactions(&vec![tx.clone()])
                        .unwrap()
                        .payload(&Payload::default())
                        .unwrap()
                        .prove(&mut prover)
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let res = block.check();
    assert!(res.is_ok());

    block.transactions_len += 1;

    let res = block.check();
    assert!(res.is_err());
}

#[test]
fn test_block_eval() {
    let block_height = 0;

    let bn = BlockNode::new()
                .meta(&Meta::default())
                .unwrap()
                .block_data(&Digest::default(), block_height)
                .unwrap();

    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .finalize(&mut hasher)
                    .unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let tx = Transaction::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .inputs(&vec![input])
                    .unwrap()
                    .outputs(&vec![output])
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let payload = Payload::default();

    let bits = 3;
    let mut prover = Prover::new(bits);

    let mut block = Block::new()
                        .meta(&Meta::default())
                        .unwrap()
                        .prev_blocks(&vec![bn.clone()])
                        .unwrap()
                        .transactions(&vec![tx.clone()])
                        .unwrap()
                        .payload(&payload)
                        .unwrap()
                        .prove(&mut prover)
                        .unwrap()
                        .finalize(&mut hasher)
                        .unwrap();

    let mut evaluator = BlockEvaluator{};

    let res = block.eval(&PayloadEvalParams::Const, &evaluator);
    assert!(res.is_ok());

    let const_res = res.unwrap();
    assert_eq!(const_res, PayloadEvalResult::Const(payload.to_string()));

    let res = block.eval_mut(&PayloadEvalMutParams::ToUppercase, &mut evaluator);
    assert!(res.is_ok());

    let to_uppercase_res = res.unwrap();

    let uppsercase_payload = payload.to_string().to_uppercase();
    assert_eq!(to_uppercase_res, PayloadEvalMutResult::ToUppercase(uppsercase_payload.clone()));
    assert_eq!(block.payload.to_string(), uppsercase_payload)
}

#[test]
fn test_block_size() {
    let block = Block::new();

    let meta_size = block.meta.get_size();
    let block_size = block.size();

    assert_eq!(meta_size, block_size);
}

#[test]
fn test_block_json() {
    let block_a = Block::new();

    let res = block_a.to_json();
    assert!(res.is_ok());

    let block_json = res.unwrap();

    let res = Block::from_json(&block_json);
    assert!(res.is_ok());

    let block_b = res.unwrap();

    assert_eq!(block_a, block_b);
}

#[test]
fn test_block_bytes() {
    let block_a = Block::new();

    let res = block_a.to_bytes();
    assert!(res.is_ok());

    let block_bytes = res.unwrap();

    let res = Block::from_bytes(&block_bytes);
    assert!(res.is_ok());

    let block_b = res.unwrap();

    assert_eq!(block_a, block_b);
}

#[test]
fn test_block_hex() {
    let block_a = Block::new();

    let res = block_a.to_hex();
    assert!(res.is_ok());

    let block_hex = res.unwrap();

    let res = Block::from_hex(&block_hex);
    assert!(res.is_ok());

    let block_b = res.unwrap();

    assert_eq!(block_a, block_b);
}

#[test]
fn test_block_store() {
    let block_height = 0;

    let bn = BlockNode::new()
                .meta(&Meta::default())
                .unwrap()
                .block_data(&Digest::default(), block_height)
                .unwrap();

    let mut hasher = Hasher{};

    let coin = Coin::new()
                    .finalize(&mut hasher)
                    .unwrap();

    let input = Input::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .coin(&coin)
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let output = Output::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .amount(&Amount::default())
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let tx = Transaction::new()
                .meta(&Meta::default())
                .unwrap()
                .inputs(&vec![input])
                .unwrap()
                .outputs(&vec![output])
                .unwrap()
                .payload(&Payload::default())
                .unwrap()
                .finalize(&mut hasher)
                .unwrap();

    let bits = 3;
    let mut prover = Prover::new(bits);

    let block = Block::new()
                    .meta(&Meta::default())
                    .unwrap()
                    .prev_blocks(&vec![bn.clone()])
                    .unwrap()
                    .transactions(&vec![tx.clone()])
                    .unwrap()
                    .payload(&Payload::default())
                    .unwrap()
                    .prove(&mut prover)
                    .unwrap()
                    .finalize(&mut hasher)
                    .unwrap();

    let mut store = Store::new();
    let res = block.store_create(&mut store);
    assert!(res.is_ok());

    let res = block.store_create(&mut store);
    assert!(res.is_err());

    let mut invalid_block = block.clone();
    invalid_block.transactions_len += 1;

    let res = invalid_block.store_create(&mut store);
    assert!(res.is_err());

    let res = Block::store_lookup(&mut store, &block.id);
    assert!(res.is_ok());
    assert!(res.unwrap());

    let unknown_id = Digest::default();

    let res = Block::store_lookup(&mut store, &unknown_id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Block::store_get(&mut store, &block.id);
    assert!(res.is_ok());

    let found_block = res.unwrap();
    assert_eq!(found_block, block);

    let res = Block::store_get(&mut store, &unknown_id);
    assert!(res.is_err());

    let mut from = Some(block.id.clone());
    let mut to = Some(block.id.clone());

    let res = Block::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Block::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = Some(block.id.clone());

    let res = Block::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    from = None;
    to = Some(block.id.clone());

    let res = Block::store_count(&mut store, from.clone(), to.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let mut from = Some(block.id.clone());
    let mut to = Some(block.id.clone());
    let mut count = None;
    let skip = 0;

    let res = Block::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    count = Some(0);

    let res = Block::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_err());

    from = None;
    to = None;
    count = None;

    let res = Block::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![block.clone()]);

    from = Some(block.id.clone());

    let res = Block::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![block.clone()]);

    from = None;
    to = Some(block.id.clone());

    let res = Block::store_list(&mut store, from.clone(), to.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = block.store_delete(&mut store);
    assert!(res.is_ok());

    let res = block.store_delete(&mut store);
    assert!(res.is_err());

    let res = Block::store_lookup(&mut store, &coin.id);
    assert!(res.is_ok());
    assert!(!res.unwrap());

    let res = Block::store_get(&mut store, &coin.id);
    assert!(res.is_err());

    from = None;
    to = None;

    let res = Block::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 0);

    let count = None;

    let res = Block::store_list(&mut store, to.clone(), from.clone(), count.clone(), skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![]);

    let res = block.store_upsert(&mut store);
    assert!(res.is_ok());

    let res = Block::store_count(&mut store, to.clone(), from.clone());
    assert!(res.is_ok());

    let count = res.unwrap();
    assert_eq!(count, 1);

    let count = None;

    let res = Block::store_list(&mut store, to, from, count, skip);
    assert!(res.is_ok());

    let list = res.unwrap();
    assert_eq!(list, vec![block.clone()]);
}