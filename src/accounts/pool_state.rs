//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use solana_program::pubkey::Pubkey;
use crate::types::RewardInfo;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;


#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PoolState {
pub discriminator: [u8; 8],
/// Bump to identify PDA
pub bump: u8,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub amm_config: Pubkey,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub owner: Pubkey,
/// Token pair of the pool, where token_mint_0 address < token_mint_1 address
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub token_mint0: Pubkey,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub token_mint1: Pubkey,
/// Token pair vault
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub token_vault0: Pubkey,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub token_vault1: Pubkey,
/// observation account key
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub observation_key: Pubkey,
/// mint0 and mint1 decimals
pub mint_decimals0: u8,
pub mint_decimals1: u8,
/// The minimum number of ticks between initialized ticks
pub tick_spacing: u16,
/// The currently in range liquidity available to the pool.
pub liquidity: u128,
/// The current price of the pool as a sqrt(token_1/token_0) Q64.64 value
pub sqrt_price_x64: u128,
/// The current tick of the pool, i.e. according to the last tick transition that was run.
pub tick_current: i32,
/// the most-recently updated index of the observations array
pub observation_index: u16,
pub observation_update_duration: u16,
/// The fee growth as a Q64.64 number, i.e. fees of token_0 and token_1 collected per
/// unit of liquidity for the entire life of the pool.
pub fee_growth_global0_x64: u128,
pub fee_growth_global1_x64: u128,
/// The amounts of token_0 and token_1 that are owed to the protocol.
pub protocol_fees_token0: u64,
pub protocol_fees_token1: u64,
/// The amounts in and out of swap token_0 and token_1
pub swap_in_amount_token0: u128,
pub swap_out_amount_token1: u128,
pub swap_in_amount_token1: u128,
pub swap_out_amount_token0: u128,
/// Bitwise representation of the state of the pool
/// bit0, 1: disable open position and increase liquidity, 0: normal
/// bit1, 1: disable decrease liquidity, 0: normal
/// bit2, 1: disable collect fee, 0: normal
/// bit3, 1: disable collect reward, 0: normal
/// bit4, 1: disable swap, 0: normal
pub status: u8,
/// Leave blank for future use
pub padding: [u8; 7],
pub reward_infos: [RewardInfo; 3],
/// Packed initialized tick array state
pub tick_array_bitmap: [u64; 16],
/// except protocol_fee and fund_fee
pub total_fees_token0: u64,
/// except protocol_fee and fund_fee
pub total_fees_claimed_token0: u64,
pub total_fees_token1: u64,
pub total_fees_claimed_token1: u64,
pub fund_fees_token0: u64,
pub fund_fees_token1: u64,
pub open_time: u64,
pub padding1: [u64; 25],
pub padding2: [u64; 32],
}


impl PoolState {
      pub const LEN: usize = 1544;
  
  
  
  #[inline(always)]
  pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
    let mut data = data;
    Self::deserialize(&mut data)
  }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for PoolState {
  type Error = std::io::Error;

  fn try_from(account_info: &solana_program::account_info::AccountInfo<'a>) -> Result<Self, Self::Error> {
      let mut data: &[u8] = &(*account_info.data).borrow();
      Self::deserialize(&mut data)
  }
}

#[cfg(feature = "fetch")]
pub fn fetch_pool_state(
  rpc: &solana_client::rpc_client::RpcClient,
  address: &Pubkey,
) -> Result<super::DecodedAccount<PoolState>, Error> {
  let accounts = fetch_all_pool_state(rpc, vec![address])?;
  Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_pool_state(
  rpc: &solana_client::rpc_client::RpcClient,
  addresses: Vec<Pubkey>,
) -> Result<Vec<super::DecodedAccount<PoolState>>, Error> {
    let accounts = rpc.get_multiple_accounts(&addresses)?;
    let mut decoded_accounts: Vec<super::DecodedAccount<PoolState>> = Vec::new();
    for i in 0..addresses.len() {
      let address = addresses[i];
      let account = accounts[i].as_ref().ok_or(format!("Account not found: {}", address))?;
      let data = PoolState::from_bytes(&account.data)?;
      decoded_accounts.push(super::DecodedAccount { address, account: account.clone(), data });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_pool_state(
  rpc: &solana_client::rpc_client::RpcClient,
  address: &Pubkey,
) -> Result<super::MaybeAccount<PoolState>, Error> {
    let accounts = fetch_all_maybe_pool_state(rpc, vec![address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_pool_state(
  rpc: &solana_client::rpc_client::RpcClient,
  addresses: Vec<Pubkey>,
) -> Result<Vec<super::MaybeAccount<PoolState>>, Error> {
    let accounts = rpc.get_multiple_accounts(&addresses)?;
    let mut decoded_accounts: Vec<super::MaybeAccount<PoolState>> = Vec::new();
    for i in 0..addresses.len() {
      let address = addresses[i];
      if let Some(account) = accounts[i].as_ref() {
        let data = PoolState::from_bytes(&account.data)?;
        decoded_accounts.push(super::MaybeAccount::Exists(super::DecodedAccount { address, account: account.clone(), data }));
      } else {
        decoded_accounts.push(super::MaybeAccount::NotFound(address));
      }
    }
  Ok(decoded_accounts)
}

  #[cfg(feature = "anchor")]
  impl anchor_lang::AccountDeserialize for PoolState {
      fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
      }
  }

  #[cfg(feature = "anchor")]
  impl anchor_lang::AccountSerialize for PoolState {}

  #[cfg(feature = "anchor")]
  impl anchor_lang::Owner for PoolState {
      fn owner() -> Pubkey {
        crate::YVAULTS_ID
      }
  }

  #[cfg(feature = "anchor-idl-build")]
  impl anchor_lang::IdlBuild for PoolState {}

  
  #[cfg(feature = "anchor-idl-build")]
  impl anchor_lang::Discriminator for PoolState {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
  }

