//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use solana_program::pubkey::Pubkey;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RewardInfo {
/// Reward state
pub reward_state: u8,
/// Reward open time
pub open_time: u64,
/// Reward end time
pub end_time: u64,
/// Reward last update time
pub last_update_time: u64,
/// Q64.64 number indicates how many tokens per second are earned per unit of liquidity.
pub emissions_per_second_x64: u128,
/// The total amount of reward emissioned
pub reward_total_emissioned: u64,
/// The total amount of claimed reward
pub reward_claimed: u64,
/// Reward token mint.
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub token_mint: Pubkey,
/// Reward vault token account.
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub token_vault: Pubkey,
/// The owner that has permission to set reward param
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub authority: Pubkey,
/// Q64.64 number that tracks the total tokens earned per unit of liquidity since the reward
/// emissions were turned on.
pub reward_growth_global_x64: u128,
}


