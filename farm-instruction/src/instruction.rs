//! All instruction types
//! These instructions represent a function what will be processed by this program

// this allows many arguments for the function parameters
#![allow(clippy::too_many_arguments)]

use {
    borsh::{BorshDeserialize, BorshSchema, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar
    },
};

/// Instructions supported by the FarmPool program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum FarmInstruction {
    ///   Set program data
    ///   [w] - writable, [s] - signer
    /// 
    ///   0. `[w]` program account.
    ///   1. `[s]` super owner of this program
    ///   2. `[]` new super owner
    ///   3. `[]` fee owner
    ///   4. `[]` allowed creator
    ///   5. `[]` AMM program id
    ///   6. `[]` farm fee
    ///   7. `[]` harvest fee numerator
    ///   8. `[]` harvest fee denominator
    ///   9. `[]` program id
    SetProgramData {
        #[allow(dead_code)]
        super_owner: Pubkey,

        #[allow(dead_code)]
        fee_owner: Pubkey,

        #[allow(dead_code)]
        allowed_creator: Pubkey,

        #[allow(dead_code)]
        amm_program_id: Pubkey,

        #[allow(dead_code)]
        farm_fee: u64,

        #[allow(dead_code)]
        harvest_fee_numerator: u64,
        
        #[allow(dead_code)]
        harvest_fee_denominator: u64,
    },

    ///   Initializes a new FarmPool.
    ///   These represent the parameters that will be included from client side
    ///   [w] - writable, [s] - signer
    /// 
    ///   0. `[w]` New FarmPool account to create.
    ///   1. `[]` authority to initialize this farm pool account
    ///   2. `[s]` Creator/Manager of this farm
    ///   3. `[w]` LP token account of this farm to store lp token
    ///   4. `[w]` reward token account of this farm to store rewards for the farmers
    ///             Creator has to transfer/deposit his reward token to this account.
    ///             only support spl tokens
    ///   5. `[]` Pool token mint address
    ///   6. `[]` Reward token mint address
    ///   7. `[]` Amm Id
    ///   8. `[]` farm program data id
    ///   9. `[]` nonce
    ///   10.'[]' start timestamp. this reflects that the farm starts at this time
    ///   11.'[]' end timestamp. this reflects that the farm ends at this time
    ///   12. `[]` program id
    InitializeFarm {
        #[allow(dead_code)]
        /// nonce
        nonce: u8,

        #[allow(dead_code)]
        /// start timestamp
        start_timestamp: u64,

        #[allow(dead_code)]
        /// end timestamp
        end_timestamp: u64,
    },

    ///   Stake Lp tokens to this farm pool
    ///   If amount is zero, only performed "harvest"
    ///   If this farm is not allowed/not started/ended, it fails
    /// 
    ///   0. `[w]` FarmPool to deposit to.
    ///   1. `[]` authority of this farm pool
    ///   2. `[s]` Depositor
    ///   3. `[]` User Farming Information Account
    ///   4. `[]` User LP token account
    ///   5. `[]` Pool LP token account
    ///   6. `[]` User reward token account
    ///   7. `[]` Pool reward token account
    ///   8. `[]` Pool LP token mint
    ///   9. `[]` fee reward ata account
    ///   10. `[]` farm program data id
    ///   11. `[]` Token program id
    ///   12. `[]` clock sysvar
    ///   13. `[]` amount
    ///   14. `[]` program id
    Deposit(u64),

    ///   Unstake LP tokens from this farm pool
    ///   Before unstake lp tokens, "harvest" works
    /// 
    ///   0. `[w]` FarmPool to withdraw to.
    ///   1. `[]` authority of this farm pool
    ///   2. `[s]` Withdrawer
    ///   3. `[]` User Farming Information Account
    ///   4. `[]` User LP token account
    ///   5. `[]` Pool LP token account
    ///   6. `[]` User reward token account
    ///   7. `[]` Pool reward token account
    ///   8. `[]` Pool LP token mint
    ///   9. `[]` fee reward ata account
    ///   10. `[]` farm program data id
    ///   11. `[]` Token program id
    ///   12. `[]` clock sysvar
    ///   13. `[]` amount
    ///   14. `[]` program id
    Withdraw(u64),

    ///   Creator can add reward to his farm 
    /// 
    ///   0. `[w]` FarmPool to add reward to.
    ///   1. `[]` authority of this farm pool
    ///   2. `[s]` creator
    ///   3. `[]` User reward token account
    ///   4. `[]` Pool reward token account
    ///   5. `[]` Pool lp token mint
    ///   6. `[]` farm program data id
    ///   7. `[]` token program id
    ///   8. `[]` clock sysvar
    ///   9. `[]` amount
    ///   10. `[]` program id
    AddReward(u64),
    
    ///   Creator has to pay farm fee (if not CRP token pairing)
    ///   So this farm can be allowed to stake/unstake/harvest
    /// 
    ///   0. `[w]` FarmPool to pay farm fee.
    ///   1. `[]` authority of this farm pool
    ///   2. `[s]` creator
    ///   3. `[]` User USDC token account
    ///   4. `[]` fee usdc ata
    ///   5. `[]` farm program data account
    ///   6. `[]` token program id
    ///   7. `[]` amount
    ///   8. `[]` program id
    PayFarmFee(u64),
}

// below functions are used to test above instructions in the rust test side
// Function's parameters


/// Creates an 'SetProgramData' instruction.
pub fn initialize_program(
    program_data_account: &Pubkey,
    super_owner: &Pubkey,
    new_super_owner: Pubkey,
    fee_owner: Pubkey,
    allowed_creator: Pubkey,
    amm_program_id: Pubkey,
    farm_fee: u64,
    harvest_fee_numerator: u64,
    harvest_fee_denominator: u64,
    program_id: &Pubkey,
) -> Instruction {
    
    let init_data = FarmInstruction::SetProgramData{
        super_owner:new_super_owner,
        fee_owner,
        allowed_creator,
        amm_program_id,
        farm_fee,
        harvest_fee_numerator,
        harvest_fee_denominator
    };
    
    let data = init_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new(*program_data_account, false),
        AccountMeta::new(*super_owner, true),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}


/// Creates an 'InitializeFarm' instruction.
pub fn initialize_farm(
    farm_id: &Pubkey,
    authority: &Pubkey,
    owner: &Pubkey,
    pool_lp_token_account: &Pubkey,
    pool_reward_token_account: &Pubkey,
    pool_mint_address: &Pubkey,
    reward_mint_address: &Pubkey,
    amm_id: &Pubkey,
    program_data_account: &Pubkey,
    nonce: u8,
    start_timestamp: u64,
    end_timestamp: u64,
    program_id: &Pubkey,
) -> Instruction {
    
    let init_data = FarmInstruction::InitializeFarm{
        nonce,
        start_timestamp,
        end_timestamp
    };
    
    let data = init_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new(*authority, false),
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new(*pool_lp_token_account, false),
        AccountMeta::new(*pool_reward_token_account, false),
        AccountMeta::new_readonly(*pool_mint_address, false),
        AccountMeta::new_readonly(*reward_mint_address, false),
        AccountMeta::new_readonly(*amm_id, false),
        AccountMeta::new_readonly(*program_data_account, false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data,
    }
}

/// Creates instructions required to deposit into a farm pool, given a farm
/// account owned by the user.
pub fn deposit(
    farm_id: &Pubkey,
    authority: &Pubkey,
    owner: &Pubkey,
    user_info_account: &Pubkey,
    user_lp_token_account: &Pubkey,
    pool_lp_token_account: &Pubkey,
    user_reward_token_account: &Pubkey,
    pool_reward_token_account: &Pubkey,
    pool_lp_mint: &Pubkey,
    fee_reward_ata: &Pubkey,
    program_data_account: &Pubkey,
    token_program_id: &Pubkey,
    amount: u64,
    program_id: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new(*user_info_account, false),
        AccountMeta::new(*user_lp_token_account, false),
        AccountMeta::new(*pool_lp_token_account, false),
        AccountMeta::new(*user_reward_token_account, false),
        AccountMeta::new(*pool_reward_token_account, false),
        AccountMeta::new(*pool_lp_mint, false),
        AccountMeta::new(*fee_reward_ata, false),
        AccountMeta::new(*program_data_account, false),
        AccountMeta::new(*token_program_id, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: FarmInstruction::Deposit(amount).try_to_vec().unwrap(),
    }
}

/// Creates a 'withdraw' instruction.
pub fn withdraw(
    farm_id: &Pubkey,
    authority: &Pubkey,
    owner: &Pubkey,
    user_info_account: &Pubkey,
    user_lp_token_account: &Pubkey,
    pool_lp_token_account: &Pubkey,
    user_reward_token_account: &Pubkey,
    pool_reward_token_account: &Pubkey,
    pool_lp_mint_info: &Pubkey,
    fee_reward_ata: &Pubkey,
    program_data_account: &Pubkey,
    token_program_id: &Pubkey,
    amount: u64,
    program_id: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new(*owner, true),
        AccountMeta::new(*user_info_account, false),
        AccountMeta::new(*user_lp_token_account, false),
        AccountMeta::new(*pool_lp_token_account, false),
        AccountMeta::new(*user_reward_token_account, false),
        AccountMeta::new(*pool_reward_token_account, false),
        AccountMeta::new(*pool_lp_mint_info, false),
        AccountMeta::new(*fee_reward_ata, false),
        AccountMeta::new(*program_data_account, false),
        AccountMeta::new(*token_program_id, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: FarmInstruction::Withdraw(amount).try_to_vec().unwrap(),
    }
}


/// Creates a instruction required to add reward into a farm pool
pub fn add_reward(
    farm_id: &Pubkey,
    authority: &Pubkey,
    owner: &Pubkey,
    user_reward_token_account: &Pubkey,
    pool_reward_token_account: &Pubkey,
    pool_lp_token_account: &Pubkey,
    pool_lp_mint_info: &Pubkey,
    program_data_account: &Pubkey,
    token_program_id: &Pubkey,
    amount: u64,
    program_id: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new(*user_reward_token_account, false),
        AccountMeta::new(*pool_reward_token_account, false),
        AccountMeta::new(*pool_lp_token_account, false),
        AccountMeta::new(*pool_lp_mint_info, false),
        AccountMeta::new(*program_data_account, false),
        AccountMeta::new(*token_program_id, false),
        AccountMeta::new_readonly(sysvar::clock::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: FarmInstruction::AddReward(amount).try_to_vec().unwrap(),
    }
}

/// Create a instruction required to pay additonal farm fee
pub fn pay_farm_fee(
    farm_id: &Pubkey,
    authority: &Pubkey,
    owner: &Pubkey,
    user_usdc_token_account: &Pubkey,
    fee_usdc_ata: &Pubkey,
    program_data_account: &Pubkey,
    token_program_id: &Pubkey,
    amount: u64,
    program_id: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*farm_id, false),
        AccountMeta::new_readonly(*authority, false),
        AccountMeta::new_readonly(*owner, true),
        AccountMeta::new(*user_usdc_token_account, false),
        AccountMeta::new(*fee_usdc_ata, false),
        AccountMeta::new(*program_data_account, false),
        AccountMeta::new(*token_program_id, false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: FarmInstruction::PayFarmFee(amount).try_to_vec().unwrap(),
    }
}