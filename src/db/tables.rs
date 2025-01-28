use super::{SqlResult, DB};

#[derive(Debug, Clone, Copy)]
pub(super) enum Table {
    Notes,
    Tags,
    NoteTags,
}

impl Table {
    /// Returns the table name as a static string
    const fn name(&self) -> &'static str {
        match self {
            Table::Notes => "notes",
            Table::Tags => "tags",
            Table::NoteTags => "note_tags",
        }
    }

    /// Returns the table schema as a static string
    const fn schema(&self) -> &'static str {
        match self {
            Table::Notes => {
                "(
                note_id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
            }
            Table::Tags => {
                "(
                tag_id INTEGER PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )"
            }
            Table::NoteTags => {
                "(
                note_id INTEGER,
                tag_id INTEGER,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (note_id, tag_id),
                FOREIGN KEY (note_id) REFERENCES notes(note_id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags(tag_id) ON DELETE CASCADE
            )"
            }
        }
    }

    /// Creates the table in the database if it doesn't exist
    pub fn create(&self, db: &DB) -> SqlResult<()> {
        let command = format!(
            "CREATE TABLE IF NOT EXISTS {} {}",
            &self.name(),
            &self.schema()
        );

        db.connection.execute(&command, [])?;

        Ok(())
    }
}
