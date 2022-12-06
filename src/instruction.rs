use borsh::{BorshDeserialize} ;
use solana_program::{program_error::ProgramError};

#[derive(BorshDeserialize)]
struct CreateNoteInstructionData {
  id: u64,
  date: String ,
  title: String,
  note: String, 
}

pub enum NoteInstruction {
  CreateNote {
    id: u64,
    date: String,
    title: String,
    note: String,
  },

  UpdateNote {
    id: u64,
    date: String,
    title: String,
    note: String,
  },

  DeleteNote {
    id: u64,
  },

}


impl NoteInstruction {
  pub fn unpack(data: &[u8])-> Result<Self,ProgramError>{

    let (&variant,rest) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;

    let payload = CreateNoteInstructionData::try_from_slice(rest).unwrap();

    Ok(match variant {
      0 => {
        Self::CreateNote { id:payload.id , date:payload.date, title:payload.title, note:payload.note }
      },
      1 => { Self::UpdateNote { id:payload.id , date:payload.date, title:payload.title, note:payload.note }},
      2 => { Self::DeleteNote { id:payload.id }},
      _ => { return  Err(ProgramError::InvalidInstructionData)}

    })
  }
}