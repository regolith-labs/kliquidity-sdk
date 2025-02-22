//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct UpdateTreasuryFeeVault {
    pub signer: solana_program::pubkey::Pubkey,

    pub global_config: solana_program::pubkey::Pubkey,

    pub fee_mint: solana_program::pubkey::Pubkey,

    pub treasury_fee_vault: solana_program::pubkey::Pubkey,

    pub treasury_fee_vault_authority: solana_program::pubkey::Pubkey,

    pub token_infos: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl UpdateTreasuryFeeVault {
    pub fn instruction(
        &self,
        args: UpdateTreasuryFeeVaultInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UpdateTreasuryFeeVaultInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.signer,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.global_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.fee_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.treasury_fee_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.treasury_fee_vault_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_infos,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.rent, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = UpdateTreasuryFeeVaultInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::YVAULTS_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateTreasuryFeeVaultInstructionData {
    discriminator: [u8; 8],
}

impl UpdateTreasuryFeeVaultInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [9, 241, 94, 91, 173, 74, 166, 119],
        }
    }
}

impl Default for UpdateTreasuryFeeVaultInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateTreasuryFeeVaultInstructionArgs {
    pub collateral_id: u16,
}

/// Instruction builder for `UpdateTreasuryFeeVault`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[writable]` global_config
///   2. `[]` fee_mint
///   3. `[writable]` treasury_fee_vault
///   4. `[]` treasury_fee_vault_authority
///   5. `[]` token_infos
///   6. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   7. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   8. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct UpdateTreasuryFeeVaultBuilder {
    signer: Option<solana_program::pubkey::Pubkey>,
    global_config: Option<solana_program::pubkey::Pubkey>,
    fee_mint: Option<solana_program::pubkey::Pubkey>,
    treasury_fee_vault: Option<solana_program::pubkey::Pubkey>,
    treasury_fee_vault_authority: Option<solana_program::pubkey::Pubkey>,
    token_infos: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    collateral_id: Option<u16>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdateTreasuryFeeVaultBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn signer(&mut self, signer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.signer = Some(signer);
        self
    }
    #[inline(always)]
    pub fn global_config(&mut self, global_config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.global_config = Some(global_config);
        self
    }
    #[inline(always)]
    pub fn fee_mint(&mut self, fee_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.fee_mint = Some(fee_mint);
        self
    }
    #[inline(always)]
    pub fn treasury_fee_vault(
        &mut self,
        treasury_fee_vault: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.treasury_fee_vault = Some(treasury_fee_vault);
        self
    }
    #[inline(always)]
    pub fn treasury_fee_vault_authority(
        &mut self,
        treasury_fee_vault_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.treasury_fee_vault_authority = Some(treasury_fee_vault_authority);
        self
    }
    #[inline(always)]
    pub fn token_infos(&mut self, token_infos: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_infos = Some(token_infos);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// `[optional account, default to 'SysvarRent111111111111111111111111111111111']`
    #[inline(always)]
    pub fn rent(&mut self, rent: solana_program::pubkey::Pubkey) -> &mut Self {
        self.rent = Some(rent);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn collateral_id(&mut self, collateral_id: u16) -> &mut Self {
        self.collateral_id = Some(collateral_id);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = UpdateTreasuryFeeVault {
            signer: self.signer.expect("signer is not set"),
            global_config: self.global_config.expect("global_config is not set"),
            fee_mint: self.fee_mint.expect("fee_mint is not set"),
            treasury_fee_vault: self
                .treasury_fee_vault
                .expect("treasury_fee_vault is not set"),
            treasury_fee_vault_authority: self
                .treasury_fee_vault_authority
                .expect("treasury_fee_vault_authority is not set"),
            token_infos: self.token_infos.expect("token_infos is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            rent: self.rent.unwrap_or(solana_program::pubkey!(
                "SysvarRent111111111111111111111111111111111"
            )),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = UpdateTreasuryFeeVaultInstructionArgs {
            collateral_id: self
                .collateral_id
                .clone()
                .expect("collateral_id is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `update_treasury_fee_vault` CPI accounts.
pub struct UpdateTreasuryFeeVaultCpiAccounts<'a, 'b> {
    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub global_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub treasury_fee_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub treasury_fee_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_infos: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `update_treasury_fee_vault` CPI instruction.
pub struct UpdateTreasuryFeeVaultCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub signer: &'b solana_program::account_info::AccountInfo<'a>,

    pub global_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub fee_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub treasury_fee_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub treasury_fee_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_infos: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: UpdateTreasuryFeeVaultInstructionArgs,
}

impl<'a, 'b> UpdateTreasuryFeeVaultCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdateTreasuryFeeVaultCpiAccounts<'a, 'b>,
        args: UpdateTreasuryFeeVaultInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            signer: accounts.signer,
            global_config: accounts.global_config,
            fee_mint: accounts.fee_mint,
            treasury_fee_vault: accounts.treasury_fee_vault,
            treasury_fee_vault_authority: accounts.treasury_fee_vault_authority,
            token_infos: accounts.token_infos,
            system_program: accounts.system_program,
            rent: accounts.rent,
            token_program: accounts.token_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(9 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.signer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.global_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.fee_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.treasury_fee_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.treasury_fee_vault_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_infos.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.rent.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = UpdateTreasuryFeeVaultInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::YVAULTS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(10 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.signer.clone());
        account_infos.push(self.global_config.clone());
        account_infos.push(self.fee_mint.clone());
        account_infos.push(self.treasury_fee_vault.clone());
        account_infos.push(self.treasury_fee_vault_authority.clone());
        account_infos.push(self.token_infos.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `UpdateTreasuryFeeVault` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` signer
///   1. `[writable]` global_config
///   2. `[]` fee_mint
///   3. `[writable]` treasury_fee_vault
///   4. `[]` treasury_fee_vault_authority
///   5. `[]` token_infos
///   6. `[]` system_program
///   7. `[]` rent
///   8. `[]` token_program
#[derive(Clone, Debug)]
pub struct UpdateTreasuryFeeVaultCpiBuilder<'a, 'b> {
    instruction: Box<UpdateTreasuryFeeVaultCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateTreasuryFeeVaultCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdateTreasuryFeeVaultCpiBuilderInstruction {
            __program: program,
            signer: None,
            global_config: None,
            fee_mint: None,
            treasury_fee_vault: None,
            treasury_fee_vault_authority: None,
            token_infos: None,
            system_program: None,
            rent: None,
            token_program: None,
            collateral_id: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn signer(
        &mut self,
        signer: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.signer = Some(signer);
        self
    }
    #[inline(always)]
    pub fn global_config(
        &mut self,
        global_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.global_config = Some(global_config);
        self
    }
    #[inline(always)]
    pub fn fee_mint(
        &mut self,
        fee_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.fee_mint = Some(fee_mint);
        self
    }
    #[inline(always)]
    pub fn treasury_fee_vault(
        &mut self,
        treasury_fee_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.treasury_fee_vault = Some(treasury_fee_vault);
        self
    }
    #[inline(always)]
    pub fn treasury_fee_vault_authority(
        &mut self,
        treasury_fee_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.treasury_fee_vault_authority = Some(treasury_fee_vault_authority);
        self
    }
    #[inline(always)]
    pub fn token_infos(
        &mut self,
        token_infos: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_infos = Some(token_infos);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn rent(&mut self, rent: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.rent = Some(rent);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn collateral_id(&mut self, collateral_id: u16) -> &mut Self {
        self.instruction.collateral_id = Some(collateral_id);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = UpdateTreasuryFeeVaultInstructionArgs {
            collateral_id: self
                .instruction
                .collateral_id
                .clone()
                .expect("collateral_id is not set"),
        };
        let instruction = UpdateTreasuryFeeVaultCpi {
            __program: self.instruction.__program,

            signer: self.instruction.signer.expect("signer is not set"),

            global_config: self
                .instruction
                .global_config
                .expect("global_config is not set"),

            fee_mint: self.instruction.fee_mint.expect("fee_mint is not set"),

            treasury_fee_vault: self
                .instruction
                .treasury_fee_vault
                .expect("treasury_fee_vault is not set"),

            treasury_fee_vault_authority: self
                .instruction
                .treasury_fee_vault_authority
                .expect("treasury_fee_vault_authority is not set"),

            token_infos: self
                .instruction
                .token_infos
                .expect("token_infos is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),

            rent: self.instruction.rent.expect("rent is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UpdateTreasuryFeeVaultCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    signer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    global_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    fee_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    treasury_fee_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    treasury_fee_vault_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_infos: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    collateral_id: Option<u16>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
