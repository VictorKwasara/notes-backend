use borsh::{BorshDeserialize, BorshSerialize} ;
use solana_program::{program_pack::{Sealed, IsInitialized}, pubkey::Pubkey};

#[derive(BorshSerialize,BorshDeserialize,Debug)]
pub struct Note {
  pub is_initialized: bool,
  pub owner: Pubkey ,
  // pub id: u8, //8
  pub date: String , // 4 + len
  pub title: String ,
  pub note: String ,
}

impl Note {
  pub fn size (&self)-> usize {
    1 + 32+ (4 + self.date.len()) + (self.title.len() + 4) + (self.note.len() + 4)
  } 
  
}

impl Sealed for Note { }

  impl IsInitialized for Note {
    fn is_initialized(&self) -> bool {
      self.is_initialized
    }
  }

#[derive(Debug,BorshDeserialize, BorshSerialize)]
pub struct Notes {
  pub is_initialized: bool,
  pub counter: u8 ,
} 

impl Notes {
  pub fn size (&self)-> usize {
    1 + 1
  } 
  
}

impl Sealed for Notes { }

  impl IsInitialized for Notes {
    fn is_initialized(&self) -> bool {
      self.is_initialized
    }
  }




