use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    account_info::{AccountInfo, next_account_info},
    msg, 
    pubkey::Pubkey, 
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts:&[AccountInfo],
    data: &[u8],
)->ProgramResult {
    msg!("Hello world") ;
Ok(())
}