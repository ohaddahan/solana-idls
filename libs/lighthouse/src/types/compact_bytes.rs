use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Eq, PartialEq, Debug, BorshSerialize, BorshDeserialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LEB128Vec<T: BorshSerialize + BorshDeserialize>(Vec<T>);

pub type CompactBytes = LEB128Vec<u8>;
