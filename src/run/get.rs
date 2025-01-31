use crate::{
    types::{Note, DB},
    GetArgs,
};

use super::RunResult;

pub fn getter(args: GetArgs) -> RunResult {
    let db = DB::new_rc()?;
    let note = Note::from_id(db, args.id.into())?;
    println!("--- {} ---", note.title());
    println!("{}", note.content());
    println!("---");
    println!("TAGS:");
    note.tags()
        .iter()
        .for_each(|tag| println!(" - {}", tag.name()));
    Ok(())
}
