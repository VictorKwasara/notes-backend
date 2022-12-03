
enum NoteInstructon {
  CreateNote {
    date: String,
    title: String,
    note: String,
  },

  UpdateNote {
    date: String,
    title: String,
    note: String,
  },

  DeleteNote {
    date: String,
    title: String,
    note: String,
  },

}

impl NoteInstructon {
  fn unpack(data: &[u8])-> Reasult<Self,()>{
    let (&variant,rest) = data.split_first()?;
    let 

  }
}