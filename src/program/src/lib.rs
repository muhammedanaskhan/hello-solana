use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

// Declare the entrypoint of the program
entrypoint!(process_instruction);

// This function is executed when your Solana program is invoked
pub fn process_instruction(
    program_id: &Pubkey,          // Public key of the program
    accounts: &[AccountInfo],     // The accounts to interact with
    instruction_data: &[u8],      // Instruction data
) -> ProgramResult {
    // Log a message to the Solana runtime
    msg!("Hello, Solana!");

    // Implement your logic here
    
    Ok(())
}
