use clap::ValueEnum;
use strum::EnumIter;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default, EnumIter, strum::Display)]
pub enum MadaraMode {
    #[default]
    Devnet,
    Sequencer,
    FullNode,
    AppChain,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq, Default, EnumIter, strum::Display)]
pub enum MadaraNetwork {
    /// Starknet mainnet
    #[default]
    Mainnet,
    /// Starknet sepolia
    Testnet,
    /// Starknet integration
    Integration,
    /// Madara devnet
    Devnet,
}
