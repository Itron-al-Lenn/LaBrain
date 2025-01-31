use std::{rc::Rc, sync::Mutex};

use crate::{
    traits::ID,
    types::{LaResult, DB},
};

use super::Error;

#[derive(ID)]
pub struct NoteId(i64);
#[derive(ID)]
pub struct TagId(i64);

#[derive(Debug)]
pub struct Note {
    id: NoteId,
    title: Rc<str>,
    content: Rc<str>,
    tags: Rc<[Tag]>,
    db: Rc<Mutex<DB>>,
}

#[derive(Debug)]
pub struct Tag {
    id: TagId,
    name: Rc<str>,
    description: Rc<str>,
    db: Rc<Mutex<DB>>,
}

impl PartialEq for Note {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Note {
    pub fn new(
        db: Rc<Mutex<DB>>,
        title: &str,
        content: &str,
        tags: Rc<[Tag]>,
    ) -> Result<Self, rusqlite::Error> {
        let mut active_db = db.lock().unwrap();
        let tx = active_db.connection.transaction()?;
        tx.execute(
            "INSERT INTO notes (title, content) VALUES (?1, ?2)",
            (title, content),
        )?;
        let id = tx.last_insert_rowid().into();
        tx.commit()?;
        Ok(Self {
            id,
            db: db.clone(),
            title: title.into(),
            content: content.into(),
            tags,
        })
    }

    pub fn from_id(db: Rc<Mutex<DB>>, id: NoteId) -> LaResult<Note> {
        let active_db = db.lock().unwrap();
        let tags: Rc<[Tag]> = Tag::from_note_id(db.clone(), id)?;

        Ok(active_db.connection.query_row(
            "SELECT title, content FROM notes WHERE note_id = ?1",
            [id.id()],
            |row| {
                Ok(Self {
                    id,
                    db: db.clone(),
                    title: row.get::<_, String>(0)?.as_str().into(),
                    content: row.get::<_, String>(1)?.as_str().into(),
                    tags,
                })
            },
        )?)
    }

    pub fn get_notes(db: Rc<Mutex<DB>>) -> LaResult<Rc<[Self]>> {
        let active_db = db.lock().unwrap();
        let mut stmt = active_db.connection.prepare("SELECT note_id FROM notes")?;

        let notes = stmt
            .query_map([], |row| {
                Ok(Note::from_id(db.clone(), row.get::<usize, i64>(0)?.into()))
            })?
            .map(|result| result?.map_err(Error::from))
            .collect::<LaResult<Rc<[Self]>>>();
        #[allow(clippy::let_and_return)]
        notes
    }

    pub fn add_tag(&mut self, tag: TagId) -> LaResult<()> {
        let mut active_db = self.db.lock().unwrap();
        let tx = active_db.connection.transaction()?;
        tx.execute(
            "INSERT INTO note_tags (note_id, tag_id) VALUES (?1, ?2)",
            [self.id_int(), tag.id()],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn id(&self) -> NoteId {
        self.id
    }

    pub fn id_int(&self) -> i64 {
        self.id().id()
    }

    pub fn tags(&self) -> Rc<[Tag]> {
        self.tags.clone()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Tag {
    pub fn new(db: Rc<Mutex<DB>>, name: &str, description: &str) -> LaResult<Self> {
        let mut active_db = db.lock().unwrap();
        let tx = active_db.connection.transaction()?;
        tx.execute(
            "INSERT INTO tags (name, description) VALUES (?1, ?2)",
            (name, description),
        )?;
        let id = tx.last_insert_rowid().into();
        tx.commit()?;

        Ok(Self {
            id,
            db: db.clone(),
            name: name.into(),
            description: description.into(),
        })
    }

    pub fn from_id(db: Rc<Mutex<DB>>, id: TagId) -> LaResult<Self> {
        let active_db = db.lock().unwrap();
        Ok(active_db.connection.query_row(
            "SELECT name, description FROM tags WHERE tag_id = ?1",
            [id.id()],
            |row| {
                Ok(Self {
                    id,
                    db: db.clone(),
                    name: row.get::<_, String>(0)?.as_str().into(),
                    description: row.get::<_, String>(1)?.as_str().into(),
                })
            },
        )?)
    }

    pub fn from_note_id(db: Rc<Mutex<DB>>, id: NoteId) -> LaResult<Rc<[Tag]>> {
        let active_db = db.lock().unwrap();
        let mut stmt = active_db
            .connection
            .prepare("SELECT tag_id FROM note_tags WHERE note_id = ?1")?;

        let tags = stmt
            .query_map([id.id()], |row| {
                Ok(Tag::from_id(db.clone(), row.get::<_, i64>(0)?.into()))
            })?
            .map(|result| result?.map_err(Error::from))
            .collect::<LaResult<Rc<[Tag]>>>();
        #[allow(clippy::let_and_return)]
        tags
    }

    pub fn id(&self) -> TagId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod test {}
