use crate::{db::DB, GetArgs};

use super::RunResult;

pub fn getter(args: GetArgs) -> RunResult {
    let db = DB::new()?;
    let note = db.get_note(args.id.into())?;
    println!("--- {} ---", note.title());
    println!("{}", note.content());
    println!("---");
    println!("TAGS:");
    note.tags()
        .into_iter()
        .for_each(|tag| println!(" - {}", tag.name()));
    Ok(())
}
