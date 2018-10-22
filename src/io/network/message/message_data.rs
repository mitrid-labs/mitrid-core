//! # MessageData
//!
//! `message_data` is the module providing the type representing a network message data.

use base::Numerical;
use base::{ConstantSize, VariableSize};
use base::Datable;
use models::coin::Coin;
use models::input::Input;
use models::output::Output;
use models::transaction::Transaction;
use models::blocknode::BlockNode;
use models::block::Block;
use models::blockgraph::BlockGraph;
use io::Node;

/// Type representing the data of a network message.
pub enum MessageData<Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{
    /// No data.
    #[repr(u8)]
    None,
    /// Node data.
    Node(Node<Ad, NP>),
    /// Nodes data
    Nodes(Vec<Node<Ad, NP>>),
    /// Coin data.
    Coin(Coin<D, Am>),
    /// Coins data.
    Coins(Vec<Coin<D, Am>>),
    /// Input data.
    Input(Input<D, Am, IP, Pk, Sig>),
    /// Inputs data.
    Inputs(Vec<Input<D, Am, IP, Pk, Sig>>),
    /// Output data.
    Output(Output<D, Pk, Am, OP>),
    /// Outputs data.
    Outputs(Vec<Output<D, Pk, Am, OP>>),
    /// Transaction data.
    Transaction(Transaction<D, Am, IP, Pk, Sig, OP, TP>),
    /// Transactions data.
    Transactions(Vec<Transaction<D, Am, IP, Pk, Sig, OP, TP>>),
    /// Blocknode data.
    BlockNode(BlockNode<D>),
    /// Blocknodes data.
    BlockNodes(Vec<BlockNode<D>>),
    /// Block data.
    Block(Block<D, Am, IP, Pk, Sig, OP, TP, BP, Pr>),
    /// Blocks data.
    Blocks(Vec<Block<D, Am, IP, Pk, Sig, OP, TP, BP, Pr>>),
    /// Blockgraph data.
    BlockGraph(BlockGraph<D, BGP>),
    /// Blockgraphs data.
    BlockGraphs(Vec<BlockGraph<D, BGP>>),
    /// Custom data.
    Custom(C),
    /// Error data.
    Error(String),
}

impl<Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> MessageData<Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   Ad: Datable + VariableSize,
            NP: Datable,
            D: Datable + ConstantSize,
            Pk: Datable + ConstantSize,
            Sig: Datable + ConstantSize,
            Pr: Datable,
            Am: Numerical,
            IP: Datable,
            OP: Datable,
            TP: Datable,
            BP: Datable,
            BGP: Datable,
            C: Datable
{
    /// Creates a `MessageData` with no data.
    pub fn message_data_none() -> MessageData<Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> {
        MessageData::None
    }
}