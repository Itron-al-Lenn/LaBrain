use crate::{
    traits::ID,
    types::{Note, DB},
};

use super::RunResult;

pub fn lister() -> RunResult {
    let db = DB::new_rc()?;
    let notes = Note::get_notes(db)?;
    println!("--- Search Results ---");
    notes
        .iter()
        .for_each(|note| println!("ID: {} Title: {}", note.id().id(), note.title()));
    Ok(())
}
