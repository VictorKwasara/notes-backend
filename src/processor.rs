use solana_program::{
    entrypoint::ProgramResult,
    account_info::{AccountInfo, next_account_info},
    msg, 
    pubkey::Pubkey, 
};
use crate::instruction::{NoteInstruction} ;

pub fn process(
    program_id: &Pubkey,
    accounts:&[AccountInfo],
    instruction_data: &[u8],
)->ProgramResult {
    let instruction = NoteInstruction::unpack(instruction_data)?;

    match instruction {
        NoteInstruction::CreateNote { id, date, title, note } => {
            create_note(program_id, accounts, title, note,date)?
        },
        _=>{}
    }
    msg!("Hello world") ;
    Ok(())
}

fn create_note (
    program_id: &Pubkey ,
    accounts: &[AccountInfo],
    title: String ,
    note: String ,
    date: String
)-> ProgramResult {

    msg!("title: {}", title);
    msg!("note: {}", note);
    msg!("date: {}", date);

Ok(())
}