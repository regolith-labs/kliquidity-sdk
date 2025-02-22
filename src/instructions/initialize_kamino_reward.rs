//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct InitializeKaminoReward {
    pub admin_authority: solana_program::pubkey::Pubkey,

    pub strategy: solana_program::pubkey::Pubkey,

    pub global_config: solana_program::pubkey::Pubkey,

    pub reward_mint: solana_program::pubkey::Pubkey,

    pub reward_vault: solana_program::pubkey::Pubkey,

    pub token_infos: solana_program::pubkey::Pubkey,

    pub base_vault_authority: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl InitializeKaminoReward {
    pub fn instruction(
        &self,
        args: InitializeKaminoRewardInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeKaminoRewardInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.strategy,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.global_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.reward_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.reward_vault,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_infos,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.base_vault_authority,
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
        let mut data = InitializeKaminoRewardInstructionData::new()
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
pub struct InitializeKaminoRewardInstructionData {
    discriminator: [u8; 8],
}

impl InitializeKaminoRewardInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [203, 212, 8, 90, 91, 118, 111, 50],
        }
    }
}

impl Default for InitializeKaminoRewardInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeKaminoRewardInstructionArgs {
    pub kamino_reward_index: u64,
    pub collateral_token: u64,
}

/// Instruction builder for `InitializeKaminoReward`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` admin_authority
///   1. `[writable]` strategy
///   2. `[]` global_config
///   3. `[]` reward_mint
///   4. `[writable, signer]` reward_vault
///   5. `[]` token_infos
///   6. `[writable]` base_vault_authority
///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   8. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   9. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct InitializeKaminoRewardBuilder {
    admin_authority: Option<solana_program::pubkey::Pubkey>,
    strategy: Option<solana_program::pubkey::Pubkey>,
    global_config: Option<solana_program::pubkey::Pubkey>,
    reward_mint: Option<solana_program::pubkey::Pubkey>,
    reward_vault: Option<solana_program::pubkey::Pubkey>,
    token_infos: Option<solana_program::pubkey::Pubkey>,
    base_vault_authority: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    kamino_reward_index: Option<u64>,
    collateral_token: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeKaminoRewardBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn admin_authority(
        &mut self,
        admin_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.admin_authority = Some(admin_authority);
        self
    }
    #[inline(always)]
    pub fn strategy(&mut self, strategy: solana_program::pubkey::Pubkey) -> &mut Self {
        self.strategy = Some(strategy);
        self
    }
    #[inline(always)]
    pub fn global_config(&mut self, global_config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.global_config = Some(global_config);
        self
    }
    #[inline(always)]
    pub fn reward_mint(&mut self, reward_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_mint = Some(reward_mint);
        self
    }
    #[inline(always)]
    pub fn reward_vault(&mut self, reward_vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.reward_vault = Some(reward_vault);
        self
    }
    #[inline(always)]
    pub fn token_infos(&mut self, token_infos: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_infos = Some(token_infos);
        self
    }
    #[inline(always)]
    pub fn base_vault_authority(
        &mut self,
        base_vault_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.base_vault_authority = Some(base_vault_authority);
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
    pub fn kamino_reward_index(&mut self, kamino_reward_index: u64) -> &mut Self {
        self.kamino_reward_index = Some(kamino_reward_index);
        self
    }
    #[inline(always)]
    pub fn collateral_token(&mut self, collateral_token: u64) -> &mut Self {
        self.collateral_token = Some(collateral_token);
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
        let accounts = InitializeKaminoReward {
            admin_authority: self.admin_authority.expect("admin_authority is not set"),
            strategy: self.strategy.expect("strategy is not set"),
            global_config: self.global_config.expect("global_config is not set"),
            reward_mint: self.reward_mint.expect("reward_mint is not set"),
            reward_vault: self.reward_vault.expect("reward_vault is not set"),
            token_infos: self.token_infos.expect("token_infos is not set"),
            base_vault_authority: self
                .base_vault_authority
                .expect("base_vault_authority is not set"),
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
        let args = InitializeKaminoRewardInstructionArgs {
            kamino_reward_index: self
                .kamino_reward_index
                .clone()
                .expect("kamino_reward_index is not set"),
            collateral_token: self
                .collateral_token
                .clone()
                .expect("collateral_token is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_kamino_reward` CPI accounts.
pub struct InitializeKaminoRewardCpiAccounts<'a, 'b> {
    pub admin_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub strategy: &'b solana_program::account_info::AccountInfo<'a>,

    pub global_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_infos: &'b solana_program::account_info::AccountInfo<'a>,

    pub base_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_kamino_reward` CPI instruction.
pub struct InitializeKaminoRewardCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub strategy: &'b solana_program::account_info::AccountInfo<'a>,

    pub global_config: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub reward_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_infos: &'b solana_program::account_info::AccountInfo<'a>,

    pub base_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeKaminoRewardInstructionArgs,
}

impl<'a, 'b> InitializeKaminoRewardCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeKaminoRewardCpiAccounts<'a, 'b>,
        args: InitializeKaminoRewardInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            admin_authority: accounts.admin_authority,
            strategy: accounts.strategy,
            global_config: accounts.global_config,
            reward_mint: accounts.reward_mint,
            reward_vault: accounts.reward_vault,
            token_infos: accounts.token_infos,
            base_vault_authority: accounts.base_vault_authority,
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
        let mut accounts = Vec::with_capacity(10 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.strategy.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.global_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.reward_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.reward_vault.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_infos.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.base_vault_authority.key,
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
        let mut data = InitializeKaminoRewardInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::YVAULTS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(11 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.admin_authority.clone());
        account_infos.push(self.strategy.clone());
        account_infos.push(self.global_config.clone());
        account_infos.push(self.reward_mint.clone());
        account_infos.push(self.reward_vault.clone());
        account_infos.push(self.token_infos.clone());
        account_infos.push(self.base_vault_authority.clone());
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

/// Instruction builder for `InitializeKaminoReward` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` admin_authority
///   1. `[writable]` strategy
///   2. `[]` global_config
///   3. `[]` reward_mint
///   4. `[writable, signer]` reward_vault
///   5. `[]` token_infos
///   6. `[writable]` base_vault_authority
///   7. `[]` system_program
///   8. `[]` rent
///   9. `[]` token_program
#[derive(Clone, Debug)]
pub struct InitializeKaminoRewardCpiBuilder<'a, 'b> {
    instruction: Box<InitializeKaminoRewardCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeKaminoRewardCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeKaminoRewardCpiBuilderInstruction {
            __program: program,
            admin_authority: None,
            strategy: None,
            global_config: None,
            reward_mint: None,
            reward_vault: None,
            token_infos: None,
            base_vault_authority: None,
            system_program: None,
            rent: None,
            token_program: None,
            kamino_reward_index: None,
            collateral_token: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn admin_authority(
        &mut self,
        admin_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.admin_authority = Some(admin_authority);
        self
    }
    #[inline(always)]
    pub fn strategy(
        &mut self,
        strategy: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.strategy = Some(strategy);
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
    pub fn reward_mint(
        &mut self,
        reward_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_mint = Some(reward_mint);
        self
    }
    #[inline(always)]
    pub fn reward_vault(
        &mut self,
        reward_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.reward_vault = Some(reward_vault);
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
    pub fn base_vault_authority(
        &mut self,
        base_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.base_vault_authority = Some(base_vault_authority);
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
    pub fn kamino_reward_index(&mut self, kamino_reward_index: u64) -> &mut Self {
        self.instruction.kamino_reward_index = Some(kamino_reward_index);
        self
    }
    #[inline(always)]
    pub fn collateral_token(&mut self, collateral_token: u64) -> &mut Self {
        self.instruction.collateral_token = Some(collateral_token);
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
        let args = InitializeKaminoRewardInstructionArgs {
            kamino_reward_index: self
                .instruction
                .kamino_reward_index
                .clone()
                .expect("kamino_reward_index is not set"),
            collateral_token: self
                .instruction
                .collateral_token
                .clone()
                .expect("collateral_token is not set"),
        };
        let instruction = InitializeKaminoRewardCpi {
            __program: self.instruction.__program,

            admin_authority: self
                .instruction
                .admin_authority
                .expect("admin_authority is not set"),

            strategy: self.instruction.strategy.expect("strategy is not set"),

            global_config: self
                .instruction
                .global_config
                .expect("global_config is not set"),

            reward_mint: self
                .instruction
                .reward_mint
                .expect("reward_mint is not set"),

            reward_vault: self
                .instruction
                .reward_vault
                .expect("reward_vault is not set"),

            token_infos: self
                .instruction
                .token_infos
                .expect("token_infos is not set"),

            base_vault_authority: self
                .instruction
                .base_vault_authority
                .expect("base_vault_authority is not set"),

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
struct InitializeKaminoRewardCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    admin_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    strategy: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    global_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    reward_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_infos: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    base_vault_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    kamino_reward_index: Option<u64>,
    collateral_token: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
