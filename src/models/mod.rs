//! # Models
//!
//! `models` is the module providing the types used throughout the framework to implement
//! block-based authenticated data structures and distributed ledgers.

/// Type used to convey the distributed ledger metadata.
pub mod meta;

/// Type used to represent a past `Transaction` `Output`.
pub mod coin;

/// Type used to bind as input one or more (generally one) `Coin` in a `Transaction`.
pub mod input;

/// Type used to represent the output of a `Transaction`.
pub mod output;

/// Type used to produce new `Output`s from one or more input `Coin`s.
pub mod transaction;

/// Type used to represent a node in the `BlockGraph`.
pub mod blocknode;

/// Type used to represent the (non-cryptographical) commitment to one or more `Transaction`s
/// in the `BlockGraph`.
pub mod block;

/// Type used to represent a graph of authenticated `Block`s, represented as `BlockNode`s.
pub mod blockgraph;

/// Type used to represent a wallet (account) in the protocol.
pub mod wallet;

pub use self::meta::Meta;
pub use self::coin::Coin;
pub use self::input::Input;
pub use self::output::Output;
pub use self::transaction::Transaction;
pub use self::blocknode::BlockNode;
pub use self::block::Block;
pub use self::blockgraph::BlockGraph;
pub use self::wallet::Wallet;