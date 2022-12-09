use borsh::{BorshDeserialize} ;
use solana_program::{program_error::ProgramError};

#[derive(BorshDeserialize)]
struct CreateNoteInstructionData {
  date: String ,
  title: String,
  note: String, 
}
#[derive(BorshDeserialize)]
struct AlterNoteInstructionData {
  id: u8,
  date: String ,
  title: String,
  note: String, 
}

pub enum NoteInstruction {
  CreateNote {
    date: String,
    title: String,
    note: String,
  },

  UpdateNote {
    id: u8,
    date: String,
    title: String,
    note: String,
  },

  DeleteNote {
    id: u8,
  },

}


impl NoteInstruction {
  pub fn unpack(data: &[u8])-> Result<Self,ProgramError>{

    let (&variant,rest) = data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
    Ok(match variant {
      0 => {
        let payload = CreateNoteInstructionData::try_from_slice(rest).unwrap();
        Self::CreateNote {date:payload.date, title:payload.title, note:payload.note }
      },
      1 => { let payload = AlterNoteInstructionData::try_from_slice(rest).unwrap();
            Self::UpdateNote { id:payload.id , date:payload.date, title:payload.title, note:payload.note }},
      2 => { 
        let payload = AlterNoteInstructionData::try_from_slice(rest).unwrap();
        Self::DeleteNote { id:payload.id }},
      _ => { return  Err(ProgramError::InvalidInstructionData)}

    })
  }
}