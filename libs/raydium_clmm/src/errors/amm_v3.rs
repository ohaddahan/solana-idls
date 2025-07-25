//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum AmmV3Error {
    /// 6000 - LOK
    #[error("LOK")]
    LOK = 0x1770,
    /// 6001 - Not approved
    #[error("Not approved")]
    NotApproved = 0x1771,
    /// 6002 - invalid update amm config flag
    #[error("invalid update amm config flag")]
    InvalidUpdateConfigFlag = 0x1772,
    /// 6003 - Account lack
    #[error("Account lack")]
    AccountLack = 0x1773,
    /// 6004 - Remove liquitity, collect fees owed and reward then you can close position account
    #[error("Remove liquitity, collect fees owed and reward then you can close position account")]
    ClosePositionErr = 0x1774,
    /// 6005 - Minting amount should be greater than 0
    #[error("Minting amount should be greater than 0")]
    ZeroMintAmount = 0x1775,
    /// 6006 - Tick out of range
    #[error("Tick out of range")]
    InvalidTickIndex = 0x1776,
    /// 6007 - The lower tick must be below the upper tick
    #[error("The lower tick must be below the upper tick")]
    TickInvalidOrder = 0x1777,
    /// 6008 - The tick must be greater, or equal to the minimum tick(-443636)
    #[error("The tick must be greater, or equal to the minimum tick(-443636)")]
    TickLowerOverflow = 0x1778,
    /// 6009 - The tick must be lesser than, or equal to the maximum tick(443636)
    #[error("The tick must be lesser than, or equal to the maximum tick(443636)")]
    TickUpperOverflow = 0x1779,
    /// 6010 - tick % tick_spacing must be zero
    #[error("tick % tick_spacing must be zero")]
    TickAndSpacingNotMatch = 0x177A,
    /// 6011 - Invalid tick array account
    #[error("Invalid tick array account")]
    InvalidTickArray = 0x177B,
    /// 6012 - Invalid tick array boundary
    #[error("Invalid tick array boundary")]
    InvalidTickArrayBoundary = 0x177C,
    /// 6013 - Square root price limit overflow
    #[error("Square root price limit overflow")]
    SqrtPriceLimitOverflow = 0x177D,
    /// 6014 - sqrt_price_x64 out of range
    #[error("sqrt_price_x64 out of range")]
    SqrtPriceX64 = 0x177E,
    /// 6015 - Liquidity sub delta L must be smaller than before
    #[error("Liquidity sub delta L must be smaller than before")]
    LiquiditySubValueErr = 0x177F,
    /// 6016 - Liquidity add delta L must be greater, or equal to before
    #[error("Liquidity add delta L must be greater, or equal to before")]
    LiquidityAddValueErr = 0x1780,
    /// 6017 - Invalid liquidity when update position
    #[error("Invalid liquidity when update position")]
    InvalidLiquidity = 0x1781,
    /// 6018 - Both token amount must not be zero while supply liquidity
    #[error("Both token amount must not be zero while supply liquidity")]
    ForbidBothZeroForSupplyLiquidity = 0x1782,
    /// 6019 - Liquidity insufficient
    #[error("Liquidity insufficient")]
    LiquidityInsufficient = 0x1783,
    /// 6020 - Transaction too old
    #[error("Transaction too old")]
    TransactionTooOld = 0x1784,
    /// 6021 - Price slippage check
    #[error("Price slippage check")]
    PriceSlippageCheck = 0x1785,
    /// 6022 - Too little output received
    #[error("Too little output received")]
    TooLittleOutputReceived = 0x1786,
    /// 6023 - Too much input paid
    #[error("Too much input paid")]
    TooMuchInputPaid = 0x1787,
    /// 6024 - Swap special amount can not be zero
    #[error("Swap special amount can not be zero")]
    ZeroAmountSpecified = 0x1788,
    /// 6025 - Input pool vault is invalid
    #[error("Input pool vault is invalid")]
    InvalidInputPoolVault = 0x1789,
    /// 6026 - Swap input or output amount is too small
    #[error("Swap input or output amount is too small")]
    TooSmallInputOrOutputAmount = 0x178A,
    /// 6027 - Not enought tick array account
    #[error("Not enought tick array account")]
    NotEnoughTickArrayAccount = 0x178B,
    /// 6028 - Invalid first tick array account
    #[error("Invalid first tick array account")]
    InvalidFirstTickArrayAccount = 0x178C,
    /// 6029 - Invalid reward index
    #[error("Invalid reward index")]
    InvalidRewardIndex = 0x178D,
    /// 6030 - The init reward token reach to the max
    #[error("The init reward token reach to the max")]
    FullRewardInfo = 0x178E,
    /// 6031 - The init reward token already in use
    #[error("The init reward token already in use")]
    RewardTokenAlreadyInUse = 0x178F,
    /// 6032 - The reward tokens must contain one of pool vault mint except the last reward
    #[error("The reward tokens must contain one of pool vault mint except the last reward")]
    ExceptRewardMint = 0x1790,
    /// 6033 - Invalid reward init param
    #[error("Invalid reward init param")]
    InvalidRewardInitParam = 0x1791,
    /// 6034 - Invalid collect reward desired amount
    #[error("Invalid collect reward desired amount")]
    InvalidRewardDesiredAmount = 0x1792,
    /// 6035 - Invalid collect reward input account number
    #[error("Invalid collect reward input account number")]
    InvalidRewardInputAccountNumber = 0x1793,
    /// 6036 - Invalid reward period
    #[error("Invalid reward period")]
    InvalidRewardPeriod = 0x1794,
    /// 6037 - Modification of emissiones is allowed within 72 hours from the end of the previous cycle
    #[error(
        "Modification of emissiones is allowed within 72 hours from the end of the previous cycle"
    )]
    NotApproveUpdateRewardEmissiones = 0x1795,
    /// 6038 - uninitialized reward info
    #[error("uninitialized reward info")]
    UnInitializedRewardInfo = 0x1796,
    /// 6039 - Not support token_2022 mint extension
    #[error("Not support token_2022 mint extension")]
    NotSupportMint = 0x1797,
    /// 6040 - Missing tickarray bitmap extension account
    #[error("Missing tickarray bitmap extension account")]
    MissingTickArrayBitmapExtensionAccount = 0x1798,
    /// 6041 - Insufficient liquidity for this direction
    #[error("Insufficient liquidity for this direction")]
    InsufficientLiquidityForDirection = 0x1799,
    /// 6042 - Max token overflow
    #[error("Max token overflow")]
    MaxTokenOverflow = 0x179A,
    /// 6043 - Calculate overflow
    #[error("Calculate overflow")]
    CalculateOverflow = 0x179B,
    /// 6044 - TransferFee calculate not match
    #[error("TransferFee calculate not match")]
    TransferFeeCalculateNotMatch = 0x179C,
}

impl solana_program_error::PrintProgramError for AmmV3Error {
    fn print<E>(&self) {
        solana_msg::msg!(&self.to_string());
    }
}

impl<T> solana_decode_error::DecodeError<T> for AmmV3Error {
    fn type_of() -> &'static str {
        "AmmV3Error"
    }
}
