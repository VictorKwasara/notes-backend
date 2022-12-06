use borsh::{BorshDeserialize, BorshSerialize} ;
use solana_program::{program_pack::{Sealed, IsInitialized}};

#[derive(BorshSerialize,BorshDeserialize)]
struct Note {
  pub is_initialized: bool,
  pub id: u64, //8
  pub date: String , // 4 + len
  pub title: String ,
  pub note: String ,
}

impl Sealed for Note { }

  impl IsInitialized for Note {
    fn is_initialized(&self) -> bool {
      self.is_initialized
    }
  }




