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
pub struct TermsSignature {
    pub discriminator: [u8; 8],
    #[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::Bytes>"))]
    pub signature: [u8; 64],
}

impl TermsSignature {
    pub const LEN: usize = 72;

    #[inline(always)]
    pub fn from_bytes(data: &[u8]) -> Result<Self, std::io::Error> {
        let mut data = data;
        Self::deserialize(&mut data)
    }
}

impl<'a> TryFrom<&solana_program::account_info::AccountInfo<'a>> for TermsSignature {
    type Error = std::io::Error;

    fn try_from(
        account_info: &solana_program::account_info::AccountInfo<'a>,
    ) -> Result<Self, Self::Error> {
        let mut data: &[u8] = &(*account_info.data).borrow();
        Self::deserialize(&mut data)
    }
}

#[cfg(feature = "fetch")]
pub fn fetch_terms_signature(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &Pubkey,
) -> Result<super::DecodedAccount<TermsSignature>, Error> {
    let accounts = fetch_all_terms_signature(rpc, vec![address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_terms_signature(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: Vec<Pubkey>,
) -> Result<Vec<super::DecodedAccount<TermsSignature>>, Error> {
    let accounts = rpc.get_multiple_accounts(&addresses)?;
    let mut decoded_accounts: Vec<super::DecodedAccount<TermsSignature>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        let account = accounts[i]
            .as_ref()
            .ok_or(format!("Account not found: {}", address))?;
        let data = TermsSignature::from_bytes(&account.data)?;
        decoded_accounts.push(super::DecodedAccount {
            address,
            account: account.clone(),
            data,
        });
    }
    Ok(decoded_accounts)
}

#[cfg(feature = "fetch")]
pub fn fetch_maybe_terms_signature(
    rpc: &solana_client::rpc_client::RpcClient,
    address: &Pubkey,
) -> Result<super::MaybeAccount<TermsSignature>, Error> {
    let accounts = fetch_all_maybe_terms_signature(rpc, vec![address])?;
    Ok(accounts[0].clone())
}

#[cfg(feature = "fetch")]
pub fn fetch_all_maybe_terms_signature(
    rpc: &solana_client::rpc_client::RpcClient,
    addresses: Vec<Pubkey>,
) -> Result<Vec<super::MaybeAccount<TermsSignature>>, Error> {
    let accounts = rpc.get_multiple_accounts(&addresses)?;
    let mut decoded_accounts: Vec<super::MaybeAccount<TermsSignature>> = Vec::new();
    for i in 0..addresses.len() {
        let address = addresses[i];
        if let Some(account) = accounts[i].as_ref() {
            let data = TermsSignature::from_bytes(&account.data)?;
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
impl anchor_lang::AccountDeserialize for TermsSignature {
    fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
        Ok(Self::deserialize(buf)?)
    }
}

#[cfg(feature = "anchor")]
impl anchor_lang::AccountSerialize for TermsSignature {}

#[cfg(feature = "anchor")]
impl anchor_lang::Owner for TermsSignature {
    fn owner() -> anchor_lang::prelude::Pubkey {
        crate::YVAULTS_ID
    }
}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::IdlBuild for TermsSignature {}

#[cfg(feature = "anchor-idl-build")]
impl anchor_lang::Discriminator for TermsSignature {
    const DISCRIMINATOR: [u8; 8] = [0; 8];
}
