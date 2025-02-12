//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct InitializeStrategy {
    pub admin_authority: solana_program::pubkey::Pubkey,

    pub global_config: solana_program::pubkey::Pubkey,
    /// Program owner also checked.
    pub pool: solana_program::pubkey::Pubkey,

    pub token_a_mint: solana_program::pubkey::Pubkey,

    pub token_b_mint: solana_program::pubkey::Pubkey,

    pub token_a_vault: solana_program::pubkey::Pubkey,

    pub token_b_vault: solana_program::pubkey::Pubkey,

    pub base_vault_authority: solana_program::pubkey::Pubkey,

    pub shares_mint: solana_program::pubkey::Pubkey,

    pub shares_mint_authority: solana_program::pubkey::Pubkey,

    pub token_infos: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,

    pub rent: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,

    pub token_a_token_program: solana_program::pubkey::Pubkey,

    pub token_b_token_program: solana_program::pubkey::Pubkey,

    pub strategy: solana_program::pubkey::Pubkey,
}

impl InitializeStrategy {
    pub fn instruction(
        &self,
        args: InitializeStrategyInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: InitializeStrategyInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(17 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin_authority,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.global_config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.pool, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_a_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_b_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_a_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_b_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.base_vault_authority,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.shares_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.shares_mint_authority,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_a_token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_b_token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.strategy,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = InitializeStrategyInstructionData::new()
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
pub struct InitializeStrategyInstructionData {
    discriminator: [u8; 8],
}

impl InitializeStrategyInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [208, 119, 144, 145, 178, 57, 105, 252],
        }
    }
}

impl Default for InitializeStrategyInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeStrategyInstructionArgs {
    pub strategy_type: u64,
    pub token_a_collateral_id: u64,
    pub token_b_collateral_id: u64,
}

/// Instruction builder for `InitializeStrategy`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` admin_authority
///   1. `[]` global_config
///   2. `[]` pool
///   3. `[]` token_a_mint
///   4. `[]` token_b_mint
///   5. `[writable]` token_a_vault
///   6. `[writable]` token_b_vault
///   7. `[writable]` base_vault_authority
///   8. `[writable]` shares_mint
///   9. `[writable]` shares_mint_authority
///   10. `[]` token_infos
///   11. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   12. `[optional]` rent (default to `SysvarRent111111111111111111111111111111111`)
///   13. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   14. `[]` token_a_token_program
///   15. `[]` token_b_token_program
///   16. `[writable]` strategy
#[derive(Clone, Debug, Default)]
pub struct InitializeStrategyBuilder {
    admin_authority: Option<solana_program::pubkey::Pubkey>,
    global_config: Option<solana_program::pubkey::Pubkey>,
    pool: Option<solana_program::pubkey::Pubkey>,
    token_a_mint: Option<solana_program::pubkey::Pubkey>,
    token_b_mint: Option<solana_program::pubkey::Pubkey>,
    token_a_vault: Option<solana_program::pubkey::Pubkey>,
    token_b_vault: Option<solana_program::pubkey::Pubkey>,
    base_vault_authority: Option<solana_program::pubkey::Pubkey>,
    shares_mint: Option<solana_program::pubkey::Pubkey>,
    shares_mint_authority: Option<solana_program::pubkey::Pubkey>,
    token_infos: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    rent: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    token_a_token_program: Option<solana_program::pubkey::Pubkey>,
    token_b_token_program: Option<solana_program::pubkey::Pubkey>,
    strategy: Option<solana_program::pubkey::Pubkey>,
    strategy_type: Option<u64>,
    token_a_collateral_id: Option<u64>,
    token_b_collateral_id: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl InitializeStrategyBuilder {
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
    pub fn global_config(&mut self, global_config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.global_config = Some(global_config);
        self
    }
    /// Program owner also checked.
    #[inline(always)]
    pub fn pool(&mut self, pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.pool = Some(pool);
        self
    }
    #[inline(always)]
    pub fn token_a_mint(&mut self, token_a_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_a_mint = Some(token_a_mint);
        self
    }
    #[inline(always)]
    pub fn token_b_mint(&mut self, token_b_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_b_mint = Some(token_b_mint);
        self
    }
    #[inline(always)]
    pub fn token_a_vault(&mut self, token_a_vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_a_vault = Some(token_a_vault);
        self
    }
    #[inline(always)]
    pub fn token_b_vault(&mut self, token_b_vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_b_vault = Some(token_b_vault);
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
    #[inline(always)]
    pub fn shares_mint(&mut self, shares_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.shares_mint = Some(shares_mint);
        self
    }
    #[inline(always)]
    pub fn shares_mint_authority(
        &mut self,
        shares_mint_authority: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.shares_mint_authority = Some(shares_mint_authority);
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
    pub fn token_a_token_program(
        &mut self,
        token_a_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_a_token_program = Some(token_a_token_program);
        self
    }
    #[inline(always)]
    pub fn token_b_token_program(
        &mut self,
        token_b_token_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.token_b_token_program = Some(token_b_token_program);
        self
    }
    #[inline(always)]
    pub fn strategy(&mut self, strategy: solana_program::pubkey::Pubkey) -> &mut Self {
        self.strategy = Some(strategy);
        self
    }
    #[inline(always)]
    pub fn strategy_type(&mut self, strategy_type: u64) -> &mut Self {
        self.strategy_type = Some(strategy_type);
        self
    }
    #[inline(always)]
    pub fn token_a_collateral_id(&mut self, token_a_collateral_id: u64) -> &mut Self {
        self.token_a_collateral_id = Some(token_a_collateral_id);
        self
    }
    #[inline(always)]
    pub fn token_b_collateral_id(&mut self, token_b_collateral_id: u64) -> &mut Self {
        self.token_b_collateral_id = Some(token_b_collateral_id);
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
        let accounts = InitializeStrategy {
            admin_authority: self.admin_authority.expect("admin_authority is not set"),
            global_config: self.global_config.expect("global_config is not set"),
            pool: self.pool.expect("pool is not set"),
            token_a_mint: self.token_a_mint.expect("token_a_mint is not set"),
            token_b_mint: self.token_b_mint.expect("token_b_mint is not set"),
            token_a_vault: self.token_a_vault.expect("token_a_vault is not set"),
            token_b_vault: self.token_b_vault.expect("token_b_vault is not set"),
            base_vault_authority: self
                .base_vault_authority
                .expect("base_vault_authority is not set"),
            shares_mint: self.shares_mint.expect("shares_mint is not set"),
            shares_mint_authority: self
                .shares_mint_authority
                .expect("shares_mint_authority is not set"),
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
            token_a_token_program: self
                .token_a_token_program
                .expect("token_a_token_program is not set"),
            token_b_token_program: self
                .token_b_token_program
                .expect("token_b_token_program is not set"),
            strategy: self.strategy.expect("strategy is not set"),
        };
        let args = InitializeStrategyInstructionArgs {
            strategy_type: self
                .strategy_type
                .clone()
                .expect("strategy_type is not set"),
            token_a_collateral_id: self
                .token_a_collateral_id
                .clone()
                .expect("token_a_collateral_id is not set"),
            token_b_collateral_id: self
                .token_b_collateral_id
                .clone()
                .expect("token_b_collateral_id is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `initialize_strategy` CPI accounts.
pub struct InitializeStrategyCpiAccounts<'a, 'b> {
    pub admin_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub global_config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program owner also checked.
    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_a_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_b_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_a_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_b_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub base_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub shares_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub shares_mint_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_infos: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_a_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_b_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub strategy: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `initialize_strategy` CPI instruction.
pub struct InitializeStrategyCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub admin_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub global_config: &'b solana_program::account_info::AccountInfo<'a>,
    /// Program owner also checked.
    pub pool: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_a_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_b_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_a_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_b_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub base_vault_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub shares_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub shares_mint_authority: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_infos: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub rent: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_a_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_b_token_program: &'b solana_program::account_info::AccountInfo<'a>,

    pub strategy: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: InitializeStrategyInstructionArgs,
}

impl<'a, 'b> InitializeStrategyCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: InitializeStrategyCpiAccounts<'a, 'b>,
        args: InitializeStrategyInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            admin_authority: accounts.admin_authority,
            global_config: accounts.global_config,
            pool: accounts.pool,
            token_a_mint: accounts.token_a_mint,
            token_b_mint: accounts.token_b_mint,
            token_a_vault: accounts.token_a_vault,
            token_b_vault: accounts.token_b_vault,
            base_vault_authority: accounts.base_vault_authority,
            shares_mint: accounts.shares_mint,
            shares_mint_authority: accounts.shares_mint_authority,
            token_infos: accounts.token_infos,
            system_program: accounts.system_program,
            rent: accounts.rent,
            token_program: accounts.token_program,
            token_a_token_program: accounts.token_a_token_program,
            token_b_token_program: accounts.token_b_token_program,
            strategy: accounts.strategy,
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
        let mut accounts = Vec::with_capacity(17 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin_authority.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.global_config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.pool.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_a_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_b_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_a_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_b_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.base_vault_authority.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.shares_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.shares_mint_authority.key,
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
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_a_token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_b_token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.strategy.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = InitializeStrategyInstructionData::new()
            .try_to_vec()
            .unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::YVAULTS_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(18 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.admin_authority.clone());
        account_infos.push(self.global_config.clone());
        account_infos.push(self.pool.clone());
        account_infos.push(self.token_a_mint.clone());
        account_infos.push(self.token_b_mint.clone());
        account_infos.push(self.token_a_vault.clone());
        account_infos.push(self.token_b_vault.clone());
        account_infos.push(self.base_vault_authority.clone());
        account_infos.push(self.shares_mint.clone());
        account_infos.push(self.shares_mint_authority.clone());
        account_infos.push(self.token_infos.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.rent.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.token_a_token_program.clone());
        account_infos.push(self.token_b_token_program.clone());
        account_infos.push(self.strategy.clone());
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

/// Instruction builder for `InitializeStrategy` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` admin_authority
///   1. `[]` global_config
///   2. `[]` pool
///   3. `[]` token_a_mint
///   4. `[]` token_b_mint
///   5. `[writable]` token_a_vault
///   6. `[writable]` token_b_vault
///   7. `[writable]` base_vault_authority
///   8. `[writable]` shares_mint
///   9. `[writable]` shares_mint_authority
///   10. `[]` token_infos
///   11. `[]` system_program
///   12. `[]` rent
///   13. `[]` token_program
///   14. `[]` token_a_token_program
///   15. `[]` token_b_token_program
///   16. `[writable]` strategy
#[derive(Clone, Debug)]
pub struct InitializeStrategyCpiBuilder<'a, 'b> {
    instruction: Box<InitializeStrategyCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> InitializeStrategyCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(InitializeStrategyCpiBuilderInstruction {
            __program: program,
            admin_authority: None,
            global_config: None,
            pool: None,
            token_a_mint: None,
            token_b_mint: None,
            token_a_vault: None,
            token_b_vault: None,
            base_vault_authority: None,
            shares_mint: None,
            shares_mint_authority: None,
            token_infos: None,
            system_program: None,
            rent: None,
            token_program: None,
            token_a_token_program: None,
            token_b_token_program: None,
            strategy: None,
            strategy_type: None,
            token_a_collateral_id: None,
            token_b_collateral_id: None,
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
    pub fn global_config(
        &mut self,
        global_config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.global_config = Some(global_config);
        self
    }
    /// Program owner also checked.
    #[inline(always)]
    pub fn pool(&mut self, pool: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.pool = Some(pool);
        self
    }
    #[inline(always)]
    pub fn token_a_mint(
        &mut self,
        token_a_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_a_mint = Some(token_a_mint);
        self
    }
    #[inline(always)]
    pub fn token_b_mint(
        &mut self,
        token_b_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_b_mint = Some(token_b_mint);
        self
    }
    #[inline(always)]
    pub fn token_a_vault(
        &mut self,
        token_a_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_a_vault = Some(token_a_vault);
        self
    }
    #[inline(always)]
    pub fn token_b_vault(
        &mut self,
        token_b_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_b_vault = Some(token_b_vault);
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
    pub fn shares_mint(
        &mut self,
        shares_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.shares_mint = Some(shares_mint);
        self
    }
    #[inline(always)]
    pub fn shares_mint_authority(
        &mut self,
        shares_mint_authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.shares_mint_authority = Some(shares_mint_authority);
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
    pub fn token_a_token_program(
        &mut self,
        token_a_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_a_token_program = Some(token_a_token_program);
        self
    }
    #[inline(always)]
    pub fn token_b_token_program(
        &mut self,
        token_b_token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_b_token_program = Some(token_b_token_program);
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
    pub fn strategy_type(&mut self, strategy_type: u64) -> &mut Self {
        self.instruction.strategy_type = Some(strategy_type);
        self
    }
    #[inline(always)]
    pub fn token_a_collateral_id(&mut self, token_a_collateral_id: u64) -> &mut Self {
        self.instruction.token_a_collateral_id = Some(token_a_collateral_id);
        self
    }
    #[inline(always)]
    pub fn token_b_collateral_id(&mut self, token_b_collateral_id: u64) -> &mut Self {
        self.instruction.token_b_collateral_id = Some(token_b_collateral_id);
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
        let args = InitializeStrategyInstructionArgs {
            strategy_type: self
                .instruction
                .strategy_type
                .clone()
                .expect("strategy_type is not set"),
            token_a_collateral_id: self
                .instruction
                .token_a_collateral_id
                .clone()
                .expect("token_a_collateral_id is not set"),
            token_b_collateral_id: self
                .instruction
                .token_b_collateral_id
                .clone()
                .expect("token_b_collateral_id is not set"),
        };
        let instruction = InitializeStrategyCpi {
            __program: self.instruction.__program,

            admin_authority: self
                .instruction
                .admin_authority
                .expect("admin_authority is not set"),

            global_config: self
                .instruction
                .global_config
                .expect("global_config is not set"),

            pool: self.instruction.pool.expect("pool is not set"),

            token_a_mint: self
                .instruction
                .token_a_mint
                .expect("token_a_mint is not set"),

            token_b_mint: self
                .instruction
                .token_b_mint
                .expect("token_b_mint is not set"),

            token_a_vault: self
                .instruction
                .token_a_vault
                .expect("token_a_vault is not set"),

            token_b_vault: self
                .instruction
                .token_b_vault
                .expect("token_b_vault is not set"),

            base_vault_authority: self
                .instruction
                .base_vault_authority
                .expect("base_vault_authority is not set"),

            shares_mint: self
                .instruction
                .shares_mint
                .expect("shares_mint is not set"),

            shares_mint_authority: self
                .instruction
                .shares_mint_authority
                .expect("shares_mint_authority is not set"),

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

            token_a_token_program: self
                .instruction
                .token_a_token_program
                .expect("token_a_token_program is not set"),

            token_b_token_program: self
                .instruction
                .token_b_token_program
                .expect("token_b_token_program is not set"),

            strategy: self.instruction.strategy.expect("strategy is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct InitializeStrategyCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    admin_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    global_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_a_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_b_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_a_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_b_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    base_vault_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    shares_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    shares_mint_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_infos: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    rent: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_a_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_b_token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    strategy: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    strategy_type: Option<u64>,
    token_a_collateral_id: Option<u64>,
    token_b_collateral_id: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
