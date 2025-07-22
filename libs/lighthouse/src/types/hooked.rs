use crate::types::DataValueAssertion;
use crate::types::{
    AccountInfoAssertion, MintAccountAssertion, StakeAccountAssertion, TokenAccountAssertion,
    UpgradeableLoaderStateAssertion,
};

use crate::types::compact_bytes::LEB128Vec;
use crate::types::compact_u64::CompactU64;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AccountDataAssertion {
    pub offset: CompactU64,
    pub assertion: DataValueAssertion,
}

pub type AccountInfoAssertions = LEB128Vec<AccountInfoAssertion>;
pub type AccountDataAssertions = LEB128Vec<AccountDataAssertion>;
pub type MintAccountAssertions = LEB128Vec<MintAccountAssertion>;
pub type TokenAccountAssertions = LEB128Vec<TokenAccountAssertion>;
pub type StakeAccountAssertions = LEB128Vec<StakeAccountAssertion>;
pub type UpgradeableLoaderStateAssertions = LEB128Vec<UpgradeableLoaderStateAssertion>;
pub type CompactBytes = LEB128Vec<u8>;
