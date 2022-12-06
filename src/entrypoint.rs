use solana_program::{
  entrypoint::ProgramResult,
  entrypoint,
  pubkey::Pubkey,
  account_info::AccountInfo,  
};
use crate::processor;

entrypoint!(process_instructions);

fn process_instructions(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8]
) -> ProgramResult {
  processor::process(program_id, accounts, instruction_data) 
}