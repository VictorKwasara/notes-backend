use solana_program::{
    entrypoint::ProgramResult,
    account_info::{AccountInfo, next_account_info},
    program::invoke_signed,
    msg, 
    pubkey::Pubkey, 
    program_error::ProgramError ,
    sysvar::{rent::Rent, Sysvar},
    system_instruction,
    borsh::try_from_slice_unchecked
};
use crate::instruction::{NoteInstruction} ;
use crate::state::{Note} ;
use borsh::BorshSerialize;

pub fn process(
    program_id: &Pubkey,
    accounts:&[AccountInfo],
    instruction_data: &[u8],
)->ProgramResult {
    let instruction = NoteInstruction::unpack(instruction_data)?;

    match instruction {
        NoteInstruction::CreateNote {date, title, note } => {
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

    let accounts_iter = &mut accounts.iter() ;

    let user = next_account_info(accounts_iter)? ;

    if !user.is_signer {
        return Err(ProgramError::MissingRequiredSignature)
    }

    let note_account = next_account_info(accounts_iter)? ;

    let (note_pda, bump_seed2) = Pubkey::find_program_address(&[user.key.as_ref(),title.as_bytes().as_ref()],program_id) ;

    if *note_account.key !=  note_pda {
       return Err(ProgramError::InvalidArgument)
    }
    let system_program = next_account_info(accounts_iter)? ;

    let note_len = 1 + 32  + (4 + date.len()) + (title.len() + 4) + (note.len() + 4) ;
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(note_len);


    invoke_signed(
    &system_instruction::create_account(
        user.key,
        note_account.key,
        rent_lamports,
        note_len.try_into().unwrap(),
        program_id,
     ),
    &[user.clone(), note_account.clone(),system_program.clone()],
    &[
        &[
           user.key.as_ref()
           ,title.as_bytes().as_ref()
           ,&[bump_seed2]
        ]
    ]
    )?;
   
    let mut note_data = try_from_slice_unchecked::<Note>(&note_account.data.borrow()).unwrap();

    note_data.title = title;
    note_data.owner = *user.key ;
    note_data.date = date;
    note_data.note = note;
    note_data.is_initialized = true;
    msg!("note data {:?}", note_data);
    
    note_data.serialize(&mut &mut note_account.data.borrow_mut()[..])?;
    Ok(())
}