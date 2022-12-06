use solana_program::{program_error::ProgramError} ;
use thiserror::Error ;

#[derive(Error,Debug)]
pub enum NoteError {
  #[error("Wrong note")]
  WrongNote
}

impl From<NoteError> for ProgramError {
  fn from (e : NoteError) -> Self {
    ProgramError::Custom(e as u32)
  }
}

