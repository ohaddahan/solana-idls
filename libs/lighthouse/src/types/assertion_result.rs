//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AssertionResult {
    U8(Option<u8>, Option<u8>, u8, bool),
    U16(Option<u16>, Option<u16>, u8, bool),
    U32(Option<u32>, Option<u32>, u8, bool),
    U64(Option<u64>, Option<u64>, u8, bool),
    U128(Option<u128>, Option<u128>, u8, bool),
    I8(Option<i8>, Option<i8>, u8, bool),
    I16(Option<i16>, Option<i16>, u8, bool),
    I32(Option<i32>, Option<i32>, u8, bool),
    I64(Option<i64>, Option<i64>, u8, bool),
    I128(Option<i128>, Option<i128>, u8, bool),
    Pubkey(Option<Pubkey>, Option<Pubkey>, u8, bool),
    Bytes(Vec<u8>, Vec<u8>, u8, bool),
    Bool(Option<bool>, Option<bool>, u8, bool),
}
