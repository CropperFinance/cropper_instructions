/// Main Entrypoint and declaration file

use solana_program::{
    account_info::{ AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::PrintProgramError,
    pubkey::Pubkey,
};
/// module declaration
/// 
/// instruction module
pub mod instruction;

// Declare and export the program's entrypoint
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the Yield Farming program was loaded into
    accounts: &[AccountInfo], // account informations
    _instruction_data: &[u8], // Instruction data
) -> ProgramResult {

    // processed successfully
    Ok(())
}
