//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PresetParameter {
    pub discriminator: [u8; 8],
    /// Bin step. Represent the price increment / decrement.
    pub bin_step: u16,
    /// Used for base fee calculation. base_fee_rate = base_factor * bin_step * 10 * 10^base_fee_power_factor
    pub base_factor: u16,
    /// Filter period determine high frequency trading time window.
    pub filter_period: u16,
    /// Decay period determine when the volatile fee start decay / decrease.
    pub decay_period: u16,
    /// Reduction factor controls the volatile fee rate decrement rate.
    pub reduction_factor: u16,
    /// Used to scale the variable fee component depending on the dynamic of the market
    pub variable_fee_control: u32,
    /// Maximum number of bin crossed can be accumulated. Used to cap volatile fee rate.
    pub max_volatility_accumulator: u32,
    /// Min bin id supported by the pool based on the configured bin step.
    pub min_bin_id: i32,
    /// Max bin id supported by the pool based on the configured bin step.
    pub max_bin_id: i32,
    /// Portion of swap fees retained by the protocol by controlling protocol_share parameter. protocol_swap_fee = protocol_share * total_swap_fee
    pub protocol_share: u16,
}

impl PresetParameter {
    pub const LEN: usize = 36;

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_account_info::AccountInfo<'a>> for PresetParameter {
    type Error = std::io::Error;

    fn try_from(account_info: &solana_account_info::AccountInfo<'a>) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "fetch")]
pub fn fetch_preset_parameter(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_pubkey::Pubkey,
) -> Result<crate::shared::DecodedAccount<PresetParameter>, std::io::Error> {
    let accounts = fetch_all_preset_parameter(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_preset_parameter(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_pubkey::Pubkey],
) -> Result<Vec<crate::shared::DecodedAccount<PresetParameter>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::DecodedAccount<PresetParameter>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i].as_ref().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Account not found: {}", address),
        ))?;
        let data = PresetParameter::from_bytes(&account.data)?;
        decoded_accounts.push(crate::shared::DecodedAccount {
            address,
            account: account.clone(),
            data,
        });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_preset_parameter(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &solana_pubkey::Pubkey,
) -> Result<crate::shared::MaybeAccount<PresetParameter>, std::io::Error> {
    let accounts = fetch_all_maybe_preset_parameter(rpc, &[*address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_preset_parameter(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: &[solana_pubkey::Pubkey],
) -> Result<Vec<crate::shared::MaybeAccount<PresetParameter>>, std::io::Error> {
    let accounts = rpc
        .get_multiple_accounts(addresses)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    let mut decoded_accounts: Vec<crate::shared::MaybeAccount<PresetParameter>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        if let Some(account) = accounts[i].as_ref() {
            let data = PresetParameter::from_bytes(&account.data)?;
            decoded_accounts.push(crate::shared::MaybeAccount::Exists(
                crate::shared::DecodedAccount {
                    address,
                    account: account.clone(),
                    data,
                },
            ));
        } else {
            decoded_accounts.push(crate::shared::MaybeAccount::NotFound(address));
        }
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for PresetParameter {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for PresetParameter {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for PresetParameter {
    fn owner() -> Pubkey {
        crate::LB_CLMM_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for PresetParameter {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for PresetParameter {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
