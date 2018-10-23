//! # Resource
//!
//! `resource` is the module providing the type representing a network resource.

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
use io::Method;

/// Type representing the data of a network message.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
#[allow(unused_attributes)]
pub enum Resource<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
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
    Resource<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
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
    /// Creates a `Resource` with no data.
    pub fn new_none() -> Self {
        Resource::None
    }

    /// Creates a `Resource` with a node.
    pub fn new_node(node: &Node<Ad, NP>) -> Result<Self> {
        node.check()?;

        Ok(Resource::Node(node.to_owned()))
    }

    /// Creates a `Resource` with a list of nodes.
    pub fn new_nodes(nodes: &Vec<Node<Ad, NP>>) -> Result<Self> {
        nodes.check()?;

        Ok(Resource::Nodes(nodes.to_owned()))
    }

    /// Creates a `Resource` with a coin.
    pub fn new_coin(coin: &Coin<D, Am>) -> Result<Self> {
        coin.check()?;

        Ok(Resource::Coin(coin.to_owned()))
    }

    /// Creates a `Resource` with a list of coins.
    pub fn new_coins(coins: &Vec<Coin<D, Am>>) -> Result<Self> {
        coins.check()?;

        Ok(Resource::Coins(coins.to_owned()))
    }

    /// Creates a `Resource` with an input.
    pub fn new_input(inp: &Input<D, Am, IP, Pk, Sig>) -> Result<Self> {
        inp.check()?;

        Ok(Resource::Input(inp.to_owned()))
    }

    /// Creates a `Resource` with a list of inputs.
    pub fn new_inputs(inps: &Vec<Input<D, Am, IP, Pk, Sig>>) -> Result<Self> {
        inps.check()?;

        Ok(Resource::Inputs(inps.to_owned()))
    }

    /// Creates a `Resource` with an output.
    pub fn new_output(out: &Output<D, Pk, Am, OP>) -> Result<Self> {
        out.check()?;

        Ok(Resource::Output(out.to_owned()))
    }

    /// Creates a `Resource` with a list of outputs.
    pub fn new_outputs(outs: &Vec<Output<D, Pk, Am, OP>>) -> Result<Self> {
        outs.check()?;

        Ok(Resource::Outputs(outs.to_owned()))
    }

    /// Creates a `Resource` with a transaction.
    pub fn new_transaction(tx: &Transaction<D, Am, IP, Pk, Sig, OP, TP>) -> Result<Self> {
        tx.check()?;

        Ok(Resource::Transaction(tx.to_owned()))
    }

    /// Creates a `Resource` with a list of transactions.
    pub fn new_transactions(txs: &Vec<Transaction<D, Am, IP, Pk, Sig, OP, TP>>) -> Result<Self> {
        txs.check()?;

        Ok(Resource::Transactions(txs.to_owned()))
    }

    /// Creates a `Resource` with a blocknode.
    pub fn new_blocknode(bn: &BlockNode<D>) -> Result<Self> {
        bn.check()?;

        Ok(Resource::BlockNode(bn.to_owned()))
    }

    /// Creates a `Resource` with a list of blocknodes.
    pub fn new_blocknodes(bns: &Vec<BlockNode<D>>) -> Result<Self> {
        bns.check()?;

        Ok(Resource::BlockNodes(bns.to_owned()))
    }

    /// Creates a `Resource` with a Block.
    pub fn new_block(b: &Block<D, Am, IP, Pk, Sig, OP, TP, BP, Pr>) -> Result<Self> {
        b.check()?;

        Ok(Resource::Block(b.to_owned()))
    }

    /// Creates a `Resource` with a list of blocks.
    pub fn new_blocks(bs: &Vec<Block<D, Am, IP, Pk, Sig, OP, TP, BP, Pr>>) -> Result<Self> {
        bs.check()?;

        Ok(Resource::Blocks(bs.to_owned()))
    }

    /// Creates a `Resource` with a blockgraph.
    pub fn new_blockgraph(bg: &BlockGraph<D, BGP>) -> Result<Self> {
        bg.check()?;

        Ok(Resource::BlockGraph(bg.to_owned()))
    }

    /// Creates a `Resource` with a list of blockgraphs.
    pub fn new_blockgraphs(bgs: &Vec<BlockGraph<D, BGP>>) -> Result<Self> {
        bgs.check()?;

        Ok(Resource::BlockGraphs(bgs.to_owned()))
    }

    /// Creates a `Resource` with custom data.
    pub fn new_custom(custom: &C) -> Result<Self> {
        custom.check()?;

        Ok(Resource::Custom(custom.to_owned()))
    }

    /// Creates a `Resource` with error data.
    pub fn new_error(error: &str) -> Self {
        Resource::Error(error.to_owned())
    }

    /// Returns if the `Resource` is an error resource.
    pub fn is_error(&self) -> Result<bool> {
        self.check()?;

        match self {
            &Resource::Error(_) => Ok(true),
            _ => Ok(false),
        }
    }

    /// Checks a `Method` against the `Resource`.
    pub fn check_method(&self, method: &Method) -> Result<()> {
        match *method as u8 {
            0 => {
                if self != &Resource::None {
                    return Err(String::from("invalid method"));
                }
            },
            1 => {
                match self {
                    &Resource::Session(_) => {},
                    _ => {
                        return Err(String::from("invalid method"));
                    },   
                }
            },
            2...9 => {
                match self {
                    &Resource::None | &Resource::Session(_) | &Resource::Custom(_) => {
                        return Err(String::from("invalid method"));
                    },
                    _ => {},
                }
            },
            10 => {
                match self {
                    &Resource::Custom(_) => {},
                    _ => {
                        return Err(String::from("invalid method"));
                    },
                }
            },
            _ => {
                return Err(String::from("invalid method"));
            }
        }

        Ok(())
    }
}

impl<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Default
    for Resource<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
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
        Resource::None
    }
}

impl<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Sizable
    for Resource<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
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
            &Resource::None => 1,
            &Resource::Node(ref node) => node.size(),
            &Resource::Session(ref session) => session.size(),
            &Resource::Nodes(ref nodes) => nodes.size(),
            &Resource::Coin(ref coin) => coin.size(),
            &Resource::Coins(ref coins) => coins.size(),
            &Resource::Input(ref input) => input.size(),
            &Resource::Inputs(ref inputs) => inputs.size(),
            &Resource::Output(ref output) => output.size(),
            &Resource::Outputs(ref outputs) => outputs.size(),
            &Resource::Transaction(ref transaction) => transaction.size(),
            &Resource::Transactions(ref transactions) => transactions.size(),
            &Resource::BlockNode(ref blocknode) => blocknode.size(),
            &Resource::BlockNodes(ref blocknodes) => blocknodes.size(),
            &Resource::Block(ref block) => block.size(),
            &Resource::Blocks(ref blocks) => blocks.size(),
            &Resource::BlockGraph(ref blockgraph) => blockgraph.size(),
            &Resource::BlockGraphs(ref blockgraphs) => blockgraphs.size(),
            &Resource::Custom(ref custom) => custom.size(),
            &Resource::Error(ref error) => error.size(),
        }
    }
}

impl<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Checkable
    for Resource<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
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
            &Resource::None => Ok(()),
            &Resource::Session(ref session) => session.check(),
            &Resource::Node(ref node) => node.check(),
            &Resource::Nodes(ref nodes) => nodes.check(),
            &Resource::Coin(ref coin) => coin.check(),
            &Resource::Coins(ref coins) => coins.check(),
            &Resource::Input(ref input) => input.check(),
            &Resource::Inputs(ref inputs) => inputs.check(),
            &Resource::Output(ref output) => output.check(),
            &Resource::Outputs(ref outputs) => outputs.check(),
            &Resource::Transaction(ref transaction) => transaction.check(),
            &Resource::Transactions(ref transactions) => transactions.check(),
            &Resource::BlockNode(ref blocknode) => blocknode.check(),
            &Resource::BlockNodes(ref blocknodes) => blocknodes.check(),
            &Resource::Block(ref block) => block.check(),
            &Resource::Blocks(ref blocks) => blocks.check(),
            &Resource::BlockGraph(ref blockgraph) => blockgraph.check(),
            &Resource::BlockGraphs(ref blockgraphs) => blockgraphs.check(),
            &Resource::Custom(ref custom) => custom.check(),
            &Resource::Error(ref error) => error.check(),
        }
    }
}

impl<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C> Serializable
    for Resource<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
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
    for Resource<S, Ad, NP, D, Pk, Sig, Pr, Am, IP, OP, TP, BP, BGP, C>
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