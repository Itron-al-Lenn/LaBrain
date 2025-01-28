mod tables;

use std::rc::Rc;

use crate::notes::{Note, NoteId, Tag, TagId};
use crate::traits::ID;

use rusqlite::Connection;
use tables::Table;

type SqlResult<T> = Result<T, rusqlite::Error>;

pub struct DB {
    pub connection: Connection,
}

impl DB {
    pub fn new() -> SqlResult<Self> {
        let db = {
            #[cfg(test)]
            {
                Self {
                    connection: Connection::open_in_memory()?,
                }
            }
            #[cfg(not(test))]
            {
                if let Some(path) =
                    directories::ProjectDirs::from("com", "Itron-al-Lenn", "laBrain")
                {
                    Self {
                        connection: Connection::open(path.data_local_dir().join("notes.db"))?,
                    }
                } else {
                    return Err(rusqlite::Error::InvalidQuery);
                }
            }
        };
        db.connection.execute("PRAGMA foreign_keys = ON", [])?;
        Table::Notes.create(&db)?;
        Table::Tags.create(&db)?;
        Table::NoteTags.create(&db)?;
        Ok(db)
    }

    pub fn new_tag(&mut self, name: &str, desc: &str) -> SqlResult<Tag> {
        if name.is_empty() {
            return Err(rusqlite::Error::InvalidParameterName(
                "Tag name cannot be empty".into(),
            ));
        }

        let tx = self.connection.transaction()?;
        tx.execute(
            "INSERT INTO tags (name, description) VALUES (?1, ?2)",
            (name, desc),
        )?;
        let id = tx.last_insert_rowid();
        tx.commit()?;

        Ok(Tag::new(id.into(), name, desc))
    }

    pub fn add_tag(&mut self, note: NoteId, tag: TagId) -> SqlResult<()> {
        let tx = self.connection.transaction()?;
        tx.execute(
            "INSERT INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            [note.id(), tag.id()],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn new_note(&mut self, title: &str, content: &str) -> SqlResult<Note> {
        if title.is_empty() {
            return Err(rusqlite::Error::InvalidParameterName(
                "Title cannot be empty".into(),
            ));
        }

        let tx = self.connection.transaction()?;
        tx.execute(
            "INSERT INTO notes (title, content) VALUES (?1, ?2)",
            (title, content),
        )?;
        let id = tx.last_insert_rowid();
        tx.commit()?;

        Ok(Note::new(id.into(), title, content, Rc::default()))
    }

    fn get_tags_of_note(&self, id: NoteId) -> SqlResult<Rc<[Tag]>> {
        let mut stmt = self
            .connection
            .prepare("SELECT tag_id FROM note_tags WHERE note_id = ?1")?;

        let tag_ids =
            stmt.query_map([id.id()], |row| self.get_tag(row.get::<_, i64>(0)?.into()))?;

        tag_ids.collect::<Result<Rc<[Tag]>, _>>()
    }

    pub fn get_note(&self, id: NoteId) -> SqlResult<Note> {
        let tags: Rc<[Tag]> = self.get_tags_of_note(id)?;

        self.connection.query_row(
            "SELECT title, content FROM notes WHERE note_id = ?1",
            [id.id()],
            |row| {
                Ok(Note::new(
                    id,
                    row.get::<_, String>(0)?.as_str(),
                    row.get::<_, String>(1)?.as_str(),
                    tags,
                ))
            },
        )
    }

    pub fn get_tag(&self, id: TagId) -> SqlResult<Tag> {
        self.connection.query_row(
            "SELECT name, description FROM tags WHERE tag_id = ?1",
            [id.id()],
            |row| {
                Ok(Tag::new(
                    id,
                    row.get::<_, String>(0)?.as_str(),
                    row.get::<_, String>(1)?.as_str(),
                ))
            },
        )
    }

    pub fn get_notes(&self) -> SqlResult<Rc<[Note]>> {
        let mut stmt = self.connection.prepare("SELECT note_id FROM notes")?;

        let notes = stmt.query_map([], |row| self.get_note(row.get::<_, i64>(0)?.into()))?;

        notes.collect::<Result<Rc<[Note]>, _>>()
    }

    pub fn close(self) -> Result<(), (Connection, rusqlite::Error)> {
        self.connection.close()
    }

    pub fn path(&self) -> Option<&str> {
        self.connection.path()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_and_retrieve_note() {
        let mut db = DB::new().expect("Failed creating connection");
        let note = db
            .new_note("Test", "This is a test note...")
            .expect("Failed to create the note");
        assert_eq!(note.title(), "Test");
        assert_eq!(note.content(), "This is a test note...");

        let note = db.get_note(note.id()).expect("Failed fetching the note");
        assert_eq!(note.title(), "Test");
        assert_eq!(note.content(), "This is a test note...");
    }

    #[test]
    fn create_and_add_tag() {
        let mut db = DB::new().expect("Failed creating connection");
        let note = db
            .new_note("Test", "This is a test note...")
            .expect("Failed to create the note");
        let tag = db.new_tag("TEST", "").expect("Failed to create the tag");
        db.add_tag(note.id(), tag.id())
            .expect("Failed adding the tag to the note");
        let note = db.get_note(note.id()).expect("Failed fetching the note");
        assert_eq!(note.tags()[0], tag);
    }
}
