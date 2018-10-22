//! # MessageData
//!
//! `message_data` is the module providing the type representing a network message data.

use base::Result;
use base::Numerical;
use base::{ConstantSize, VariableSize};
use base::Sizable;
use base::Checkable;
use base::Serializable;
use base::Datable;
use models::coin::Coin;
use models::input::Input;
use models::output::Output;
use models::transaction::Transaction;
use models::blocknode::BlockNode;
use models::block::Block;
use models::blockgraph::BlockGraph;
use io::Session;
use io::Node;

/// Type representing the data of a network message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum MessageData<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            Ad: Datable + VariableSize,
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
    /// Session.
    Session(Session<S>),
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

impl<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    MessageData<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable, Ad: Datable + VariableSize,
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
    pub fn new_none() -> Self {
        MessageData::None
    }

    /// Creates a `MessageData` with a node.
    pub fn new_node(node: &Node<Ad, NP>) -> Result<Self> {
        node.check()?;

        Ok(MessageData::Node(node.to_owned()))
    }

    /// Creates a `MessageData` with a list of nodes.
    pub fn new_nodes(nodes: &Vec<Node<Ad, NP>>) -> Result<Self> {
        nodes.check()?;

        Ok(MessageData::Nodes(nodes.to_owned()))
    }

    /// Creates a `MessageData` with a coin.
    pub fn new_coin(coin: &Coin<D, Am>) -> Result<Self> {
        coin.check()?;

        Ok(MessageData::Coin(coin.to_owned()))
    }

    /// Creates a `MessageData` with a list of coins.
    pub fn new_coins(coins: &Vec<Coin<D, Am>>) -> Result<Self> {
        coins.check()?;

        Ok(MessageData::Coins(coins.to_owned()))
    }

    /// Creates a `MessageData` with an input.
    pub fn new_input(inp: &Input<D, Am, IP, Pk, Sig>) -> Result<Self> {
        inp.check()?;

        Ok(MessageData::Input(inp.to_owned()))
    }

    /// Creates a `MessageData` with a list of inputs.
    pub fn new_inputs(inps: &Vec<Input<D, Am, IP, Pk, Sig>>) -> Result<Self> {
        inps.check()?;

        Ok(MessageData::Inputs(inps.to_owned()))
    }

    /// Creates a `MessageData` with an output.
    pub fn new_output(out: &Output<D, Pk, Am, OP>) -> Result<Self> {
        out.check()?;

        Ok(MessageData::Output(out.to_owned()))
    }

    /// Creates a `MessageData` with a list of outputs.
    pub fn new_outputs(outs: &Vec<Output<D, Pk, Am, OP>>) -> Result<Self> {
        outs.check()?;

        Ok(MessageData::Outputs(outs.to_owned()))
    }

    /// Creates a `MessageData` with a transaction.
    pub fn new_transaction(tx: &Transaction<D, Am, IP, Pk, Sig, OP, TP>) -> Result<Self> {
        tx.check()?;

        Ok(MessageData::Transaction(tx.to_owned()))
    }

    /// Creates a `MessageData` with a list of transactions.
    pub fn new_transactions(txs: &Vec<Transaction<D, Am, IP, Pk, Sig, OP, TP>>) -> Result<Self> {
        txs.check()?;

        Ok(MessageData::Transactions(txs.to_owned()))
    }

    /// Creates a `MessageData` with a blocknode.
    pub fn new_blocknode(bn: &BlockNode<D>) -> Result<Self> {
        bn.check()?;

        Ok(MessageData::BlockNode(bn.to_owned()))
    }

    /// Creates a `MessageData` with a list of blocknodes.
    pub fn new_blocknodes(bns: &Vec<BlockNode<D>>) -> Result<Self> {
        bns.check()?;

        Ok(MessageData::BlockNodes(bns.to_owned()))
    }

    /// Creates a `MessageData` with a Block.
    pub fn new_block(b: &Block<D, Am, IP, Pk, Sig, OP, TP, BP, Pr>) -> Result<Self> {
        b.check()?;

        Ok(MessageData::Block(b.to_owned()))
    }

    /// Creates a `MessageData` with a list of blocks.
    pub fn new_blocks(bs: &Vec<Block<D, Am, IP, Pk, Sig, OP, TP, BP, Pr>>) -> Result<Self> {
        bs.check()?;

        Ok(MessageData::Blocks(bs.to_owned()))
    }

    /// Creates a `MessageData` with a blockgraph.
    pub fn new_blockgraph(bg: &BlockGraph<D, BGP>) -> Result<Self> {
        bg.check()?;

        Ok(MessageData::BlockGraph(bg.to_owned()))
    }

    /// Creates a `MessageData` with a list of blockgraphs.
    pub fn new_blockgraphs(bgs: &Vec<BlockGraph<D, BGP>>) -> Result<Self> {
        bgs.check()?;

        Ok(MessageData::BlockGraphs(bgs.to_owned()))
    }

    /// Creates a `MessageData` with custom data.
    pub fn new_custom(custom: &C) -> Result<Self> {
        custom.check()?;

        Ok(MessageData::Custom(custom.to_owned()))
    }

    /// Creates a `MessageData` with error data.
    pub fn new_error(error: &str) -> Self {
        MessageData::Error(error.to_owned())
    }
}

impl<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Default
    for MessageData<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            Ad: Datable + VariableSize,
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
    fn default() -> Self {
        MessageData::None
    }
}

impl<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Sizable
    for MessageData<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            Ad: Datable + VariableSize,
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
    fn size(&self) -> u64 {
        match self {
            &MessageData::None => 1,
            &MessageData::Node(ref node) => node.size(),
            &MessageData::Session(ref session) => session.size(),
            &MessageData::Nodes(ref nodes) => nodes.size(),
            &MessageData::Coin(ref coin) => coin.size(),
            &MessageData::Coins(ref coins) => coins.size(),
            &MessageData::Input(ref input) => input.size(),
            &MessageData::Inputs(ref inputs) => inputs.size(),
            &MessageData::Output(ref output) => output.size(),
            &MessageData::Outputs(ref outputs) => outputs.size(),
            &MessageData::Transaction(ref transaction) => transaction.size(),
            &MessageData::Transactions(ref transactions) => transactions.size(),
            &MessageData::BlockNode(ref blocknode) => blocknode.size(),
            &MessageData::BlockNodes(ref blocknodes) => blocknodes.size(),
            &MessageData::Block(ref block) => block.size(),
            &MessageData::Blocks(ref blocks) => blocks.size(),
            &MessageData::BlockGraph(ref blockgraph) => blockgraph.size(),
            &MessageData::BlockGraphs(ref blockgraphs) => blockgraphs.size(),
            &MessageData::Custom(ref custom) => custom.size(),
            &MessageData::Error(ref error) => error.size(),
        }
    }
}

impl<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Checkable
    for MessageData<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            Ad: Datable + VariableSize,
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
    fn check(&self) -> Result<()> {
        match self {
            &MessageData::None => Ok(()),
            &MessageData::Session(ref session) => session.check(),
            &MessageData::Node(ref node) => node.check(),
            &MessageData::Nodes(ref nodes) => nodes.check(),
            &MessageData::Coin(ref coin) => coin.check(),
            &MessageData::Coins(ref coins) => coins.check(),
            &MessageData::Input(ref input) => input.check(),
            &MessageData::Inputs(ref inputs) => inputs.check(),
            &MessageData::Output(ref output) => output.check(),
            &MessageData::Outputs(ref outputs) => outputs.check(),
            &MessageData::Transaction(ref transaction) => transaction.check(),
            &MessageData::Transactions(ref transactions) => transactions.check(),
            &MessageData::BlockNode(ref blocknode) => blocknode.check(),
            &MessageData::BlockNodes(ref blocknodes) => blocknodes.check(),
            &MessageData::Block(ref block) => block.check(),
            &MessageData::Blocks(ref blocks) => blocks.check(),
            &MessageData::BlockGraph(ref blockgraph) => blockgraph.check(),
            &MessageData::BlockGraphs(ref blockgraphs) => blockgraphs.check(),
            &MessageData::Custom(ref custom) => custom.check(),
            &MessageData::Error(ref error) => error.check(),
        }
    }
}

impl<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Serializable
    for MessageData<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable + Serializable,
            Ad: Datable + VariableSize + Serializable,
            NP: Datable + Serializable,
            D: Datable + ConstantSize + Serializable,
            Pk: Datable + ConstantSize + Serializable,
            Sig: Datable + ConstantSize + Serializable,
            Pr: Datable + Serializable,
            Am: Numerical + Serializable,
            IP: Datable + Serializable,
            OP: Datable + Serializable,
            TP: Datable + Serializable,
            BP: Datable + Serializable,
            BGP: Datable + Serializable,
            C: Datable + Serializable
{}

impl<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Datable
    for MessageData<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
    where   S: Datable,
            Ad: Datable + VariableSize,
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
{}