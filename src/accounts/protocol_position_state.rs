//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProtocolPositionState {
    pub discriminator: [u8; 8],
    /// Bump to identify PDA
    pub bump: u8,
    /// The ID of the pool with which this token is connected
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
    )]
    pub pool_id: Pubkey,
    /// The lower bound tick of the position
    pub tick_lower_index: i32,
    /// The upper bound tick of the position
    pub tick_upper_index: i32,
    /// The amount of liquidity owned by this position
    pub liquidity: u128,
    /// The token_0 fee growth per unit of liquidity as of the last update to liquidity or fees owed
    pub fee_growth_inside0_last_x64: u128,
    /// The token_1 fee growth per unit of liquidity as of the last update to liquidity or fees owed
    pub fee_growth_inside1_last_x64: u128,
    /// The fees owed to the position owner in token_0
    pub token_fees_owed0: u64,
    /// The fees owed to the position owner in token_1
    pub token_fees_owed1: u64,
    /// The reward growth per unit of liquidity as of the last update to liquidity
    pub reward_growth_inside: [u128; 3],
    pub padding: [u64; 8],
}

impl ProtocolPositionState {
    pub const LEN: usize = 225;

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for ProtocolPositionState {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "fetch")]
pub fn fetch_protocol_position_state(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &Pubkey,
) -> Result<super::DecodedAccount<ProtocolPositionState>, Error> {
    let accounts = fetch_all_protocol_position_state(rpc, vec![address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_protocol_position_state(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: Vec<Pubkey>,
) -> Result<Vec<super::DecodedAccount<ProtocolPositionState>>, Error> {
    let accounts = rpc.get_multiple_accounts(&addresses)?;
    let mut decoded_accounts: Vec<super::DecodedAccount<ProtocolPositionState>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i]
            .as_ref()
            .ok_or(format!("Account not found: {}", address))?;
        let data = ProtocolPositionState::from_bytes(&account.data)?;
        decoded_accounts.push(super::DecodedAccount {
            address,
            account: account.clone(),
            data,
        });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_protocol_position_state(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &Pubkey,
) -> Result<super::MaybeAccount<ProtocolPositionState>, Error> {
    let accounts = fetch_all_maybe_protocol_position_state(rpc, vec![address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_protocol_position_state(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: Vec<Pubkey>,
) -> Result<Vec<super::MaybeAccount<ProtocolPositionState>>, Error> {
    let accounts = rpc.get_multiple_accounts(&addresses)?;
    let mut decoded_accounts: Vec<super::MaybeAccount<ProtocolPositionState>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        if let Some(account) = accounts[i].as_ref() {
            let data = ProtocolPositionState::from_bytes(&account.data)?;
            decoded_accounts.push(super::MaybeAccount::Exists(super::DecodedAccount {
                address,
                account: account.clone(),
                data,
            }));
        } else {
            decoded_accounts.push(super::MaybeAccount::NotFound(address));
        }
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountDeserialize for ProtocolPositionState {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for ProtocolPositionState {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for ProtocolPositionState {
    fn owner() -> Pubkey {
        crate::YVAULTS_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for ProtocolPositionState {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for ProtocolPositionState {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
