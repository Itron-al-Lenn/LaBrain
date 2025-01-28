use crate::{db::DB, traits::ID};

use super::RunResult;

pub fn lister() -> RunResult {
    let db = DB::new()?;
    let notes = db.get_notes()?;
    println!("--- Search Results ---");
    notes
        .into_iter()
        .for_each(|note| println!("ID: {} Title: {}", note.id().id(), note.title()));
    Ok(())
}
