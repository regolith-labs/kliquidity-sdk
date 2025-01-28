//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshSerialize;
use borsh::BorshDeserialize;

/// Accounts.
pub struct UpdateCollateralInfo {
      
              
          pub admin_authority: solana_program::pubkey::Pubkey,
          
              
          pub global_config: solana_program::pubkey::Pubkey,
          
              
          pub token_infos: solana_program::pubkey::Pubkey,
      }

impl UpdateCollateralInfo {
  pub fn instruction(&self, args: UpdateCollateralInfoInstructionArgs) -> solana_program::instruction::Instruction {
    self.instruction_with_remaining_accounts(args, &[])
  }
  #[allow(clippy::vec_init_then_push)]
  pub fn instruction_with_remaining_accounts(&self, args: UpdateCollateralInfoInstructionArgs, remaining_accounts: &[solana_program::instruction::AccountMeta]) -> solana_program::instruction::Instruction {
    let mut accounts = Vec::with_capacity(3+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            self.admin_authority,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.global_config,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_infos,
            false
          ));
                      accounts.extend_from_slice(remaining_accounts);
    let mut data = UpdateCollateralInfoInstructionData::new().try_to_vec().unwrap();
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
 pub struct UpdateCollateralInfoInstructionData {
            discriminator: [u8; 8],
                        }

impl UpdateCollateralInfoInstructionData {
  pub fn new() -> Self {
    Self {
                        discriminator: [76, 94, 131, 44, 137, 61, 161, 110],
                                                            }
  }
}

impl Default for UpdateCollateralInfoInstructionData {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
 pub struct UpdateCollateralInfoInstructionArgs {
                  pub index: u64,
                pub mode: u64,
                pub value: [u8; 32],
      }


/// Instruction builder for `UpdateCollateralInfo`.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` admin_authority
          ///   1. `[]` global_config
                ///   2. `[writable]` token_infos
#[derive(Clone, Debug, Default)]
pub struct UpdateCollateralInfoBuilder {
            admin_authority: Option<solana_program::pubkey::Pubkey>,
                global_config: Option<solana_program::pubkey::Pubkey>,
                token_infos: Option<solana_program::pubkey::Pubkey>,
                        index: Option<u64>,
                mode: Option<u64>,
                value: Option<[u8; 32]>,
        __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdateCollateralInfoBuilder {
  pub fn new() -> Self {
    Self::default()
  }
            #[inline(always)]
    pub fn admin_authority(&mut self, admin_authority: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.admin_authority = Some(admin_authority);
                    self
    }
            #[inline(always)]
    pub fn global_config(&mut self, global_config: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.global_config = Some(global_config);
                    self
    }
            #[inline(always)]
    pub fn token_infos(&mut self, token_infos: solana_program::pubkey::Pubkey) -> &mut Self {
                        self.token_infos = Some(token_infos);
                    self
    }
                    #[inline(always)]
      pub fn index(&mut self, index: u64) -> &mut Self {
        self.index = Some(index);
        self
      }
                #[inline(always)]
      pub fn mode(&mut self, mode: u64) -> &mut Self {
        self.mode = Some(mode);
        self
      }
                #[inline(always)]
      pub fn value(&mut self, value: [u8; 32]) -> &mut Self {
        self.value = Some(value);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: solana_program::instruction::AccountMeta) -> &mut Self {
    self.__remaining_accounts.push(account);
    self
  }
  /// Add additional accounts to the instruction.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[solana_program::instruction::AccountMeta]) -> &mut Self {
    self.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[allow(clippy::clone_on_copy)]
  pub fn instruction(&self) -> solana_program::instruction::Instruction {
    let accounts = UpdateCollateralInfo {
                              admin_authority: self.admin_authority.expect("admin_authority is not set"),
                                        global_config: self.global_config.expect("global_config is not set"),
                                        token_infos: self.token_infos.expect("token_infos is not set"),
                      };
          let args = UpdateCollateralInfoInstructionArgs {
                                                              index: self.index.clone().expect("index is not set"),
                                                                  mode: self.mode.clone().expect("mode is not set"),
                                                                  value: self.value.clone().expect("value is not set"),
                                    };
    
    accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
  }
}

  /// `update_collateral_info` CPI accounts.
  pub struct UpdateCollateralInfoCpiAccounts<'a, 'b> {
          
                    
              pub admin_authority: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub global_config: &'b solana_program::account_info::AccountInfo<'a>,
                
                    
              pub token_infos: &'b solana_program::account_info::AccountInfo<'a>,
            }

/// `update_collateral_info` CPI instruction.
pub struct UpdateCollateralInfoCpi<'a, 'b> {
  /// The program to invoke.
  pub __program: &'b solana_program::account_info::AccountInfo<'a>,
      
              
          pub admin_authority: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub global_config: &'b solana_program::account_info::AccountInfo<'a>,
          
              
          pub token_infos: &'b solana_program::account_info::AccountInfo<'a>,
            /// The arguments for the instruction.
    pub __args: UpdateCollateralInfoInstructionArgs,
  }

impl<'a, 'b> UpdateCollateralInfoCpi<'a, 'b> {
  pub fn new(
    program: &'b solana_program::account_info::AccountInfo<'a>,
          accounts: UpdateCollateralInfoCpiAccounts<'a, 'b>,
              args: UpdateCollateralInfoInstructionArgs,
      ) -> Self {
    Self {
      __program: program,
              admin_authority: accounts.admin_authority,
              global_config: accounts.global_config,
              token_infos: accounts.token_infos,
                    __args: args,
          }
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], &[])
  }
  #[inline(always)]
  pub fn invoke_with_remaining_accounts(&self, remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
  }
  #[inline(always)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed_with_remaining_accounts(
    &self,
    signers_seeds: &[&[&[u8]]],
    remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]
  ) -> solana_program::entrypoint::ProgramResult {
    let mut accounts = Vec::with_capacity(3+ remaining_accounts.len());
                            accounts.push(solana_program::instruction::AccountMeta::new(
            *self.admin_authority.key,
            true
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.global_config.key,
            false
          ));
                                          accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_infos.key,
            false
          ));
                      remaining_accounts.iter().for_each(|remaining_account| {
      accounts.push(solana_program::instruction::AccountMeta {
          pubkey: *remaining_account.0.key,
          is_signer: remaining_account.1,
          is_writable: remaining_account.2,
      })
    });
    let mut data = UpdateCollateralInfoInstructionData::new().try_to_vec().unwrap();
          let mut args = self.__args.try_to_vec().unwrap();
      data.append(&mut args);
    
    let instruction = solana_program::instruction::Instruction {
      program_id: crate::YVAULTS_ID,
      accounts,
      data,
    };
    let mut account_infos = Vec::with_capacity(4 + remaining_accounts.len());
    account_infos.push(self.__program.clone());
                  account_infos.push(self.admin_authority.clone());
                        account_infos.push(self.global_config.clone());
                        account_infos.push(self.token_infos.clone());
              remaining_accounts.iter().for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

    if signers_seeds.is_empty() {
      solana_program::program::invoke(&instruction, &account_infos)
    } else {
      solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
    }
  }
}

/// Instruction builder for `UpdateCollateralInfo` via CPI.
///
/// ### Accounts:
///
                      ///   0. `[writable, signer]` admin_authority
          ///   1. `[]` global_config
                ///   2. `[writable]` token_infos
#[derive(Clone, Debug)]
pub struct UpdateCollateralInfoCpiBuilder<'a, 'b> {
  instruction: Box<UpdateCollateralInfoCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateCollateralInfoCpiBuilder<'a, 'b> {
  pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
    let instruction = Box::new(UpdateCollateralInfoCpiBuilderInstruction {
      __program: program,
              admin_authority: None,
              global_config: None,
              token_infos: None,
                                            index: None,
                                mode: None,
                                value: None,
                    __remaining_accounts: Vec::new(),
    });
    Self { instruction }
  }
      #[inline(always)]
    pub fn admin_authority(&mut self, admin_authority: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.admin_authority = Some(admin_authority);
                    self
    }
      #[inline(always)]
    pub fn global_config(&mut self, global_config: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.global_config = Some(global_config);
                    self
    }
      #[inline(always)]
    pub fn token_infos(&mut self, token_infos: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
                        self.instruction.token_infos = Some(token_infos);
                    self
    }
                    #[inline(always)]
      pub fn index(&mut self, index: u64) -> &mut Self {
        self.instruction.index = Some(index);
        self
      }
                #[inline(always)]
      pub fn mode(&mut self, mode: u64) -> &mut Self {
        self.instruction.mode = Some(mode);
        self
      }
                #[inline(always)]
      pub fn value(&mut self, value: [u8; 32]) -> &mut Self {
        self.instruction.value = Some(value);
        self
      }
        /// Add an additional account to the instruction.
  #[inline(always)]
  pub fn add_remaining_account(&mut self, account: &'b solana_program::account_info::AccountInfo<'a>, is_writable: bool, is_signer: bool) -> &mut Self {
    self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
    self
  }
  /// Add additional accounts to the instruction.
  ///
  /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
  /// and a `bool` indicating whether the account is a signer or not.
  #[inline(always)]
  pub fn add_remaining_accounts(&mut self, accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)]) -> &mut Self {
    self.instruction.__remaining_accounts.extend_from_slice(accounts);
    self
  }
  #[inline(always)]
  pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
    self.invoke_signed(&[])
  }
  #[allow(clippy::clone_on_copy)]
  #[allow(clippy::vec_init_then_push)]
  pub fn invoke_signed(&self, signers_seeds: &[&[&[u8]]]) -> solana_program::entrypoint::ProgramResult {
          let args = UpdateCollateralInfoInstructionArgs {
                                                              index: self.instruction.index.clone().expect("index is not set"),
                                                                  mode: self.instruction.mode.clone().expect("mode is not set"),
                                                                  value: self.instruction.value.clone().expect("value is not set"),
                                    };
        let instruction = UpdateCollateralInfoCpi {
        __program: self.instruction.__program,
                  
          admin_authority: self.instruction.admin_authority.expect("admin_authority is not set"),
                  
          global_config: self.instruction.global_config.expect("global_config is not set"),
                  
          token_infos: self.instruction.token_infos.expect("token_infos is not set"),
                          __args: args,
            };
    instruction.invoke_signed_with_remaining_accounts(signers_seeds, &self.instruction.__remaining_accounts)
  }
}

#[derive(Clone, Debug)]
struct UpdateCollateralInfoCpiBuilderInstruction<'a, 'b> {
  __program: &'b solana_program::account_info::AccountInfo<'a>,
            admin_authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                global_config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                token_infos: Option<&'b solana_program::account_info::AccountInfo<'a>>,
                        index: Option<u64>,
                mode: Option<u64>,
                value: Option<[u8; 32]>,
        /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
  __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}

