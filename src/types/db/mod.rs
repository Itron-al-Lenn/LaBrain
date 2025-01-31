mod tables;

use std::rc::Rc;
use std::{collections::HashSet, sync::Mutex};

use crate::{
    traits::ID,
    types::{Error, LaResult, NoteId, Tag},
};

use rusqlite::Connection;
use tables::Table;

#[derive(Debug)]
pub struct DB {
    pub connection: Connection,
    active_notes: HashSet<NoteId>,
}

impl DB {
    pub fn new() -> LaResult<Self> {
        let db = {
            #[cfg(test)]
            {
                Self {
                    connection: Connection::open_in_memory()?,
                    active_notes: HashSet::default(),
                }
            }
            #[cfg(not(test))]
            {
                if let Some(path) =
                    directories::ProjectDirs::from("com", "Itron-al-Lenn", "laBrain")
                {
                    Self {
                        connection: Connection::open(path.data_local_dir().join("notes.db"))?,
                        active_notes: HashSet::default(),
                    }
                } else {
                    return Err(Error::Dir);
                }
            }
        };
        db.connection.execute("PRAGMA foreign_keys = ON", [])?;
        Table::Notes.create(&db)?;
        Table::Notes.create_triggers(&db)?;
        Table::Tags.create(&db)?;
        Table::NoteTags.create(&db)?;
        Ok(db)
    }

    pub fn new_rc() -> LaResult<Rc<Mutex<Self>>> {
        Ok(Rc::new(Mutex::new(Self::new()?)))
    }

    pub fn path(&self) -> Option<&str> {
        self.connection.path()
    }
}

#[cfg(test)]
mod test {}
