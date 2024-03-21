///`Ask(uint256,uint256,uint256,uint256,uint256,address,bytes)`
#[derive(
    Clone,
    ::ethers::contract::EthAbiType,
    ::ethers::contract::EthAbiCodec,
    serde::Serialize,
    serde::Deserialize,
    Default,
    Debug,
    PartialEq,
    Eq,
    Hash
)]
pub struct Ask {
    pub market_id: ::ethers::core::types::U256,
    pub reward: ::ethers::core::types::U256,
    pub expiry: ::ethers::core::types::U256,
    pub time_taken_for_proof_generation: ::ethers::core::types::U256,
    pub deadline: ::ethers::core::types::U256,
    pub refund_address: ::ethers::core::types::Address,
    pub prover_data: ::ethers::core::types::Bytes,
}
