use std::rc::Rc;

use crate::traits::ID;

#[derive(ID)]
pub struct NoteId(i64);
#[derive(ID)]
pub struct TagId(i64);

#[derive(PartialEq, Debug)]
pub struct Note {
    id: NoteId,
    title: Rc<str>,
    content: Rc<str>,
    tags: Rc<[Tag]>,
}

#[derive(PartialEq, Debug)]
pub struct Tag {
    id: TagId,
    name: Rc<str>,
    description: Rc<str>,
}

impl Note {
    pub fn new(id: NoteId, title: &str, content: &str, tags: Rc<[Tag]>) -> Self {
        Self {
            id,
            title: title.into(),
            content: content.into(),
            tags,
        }
    }

    pub fn id(&self) -> NoteId {
        self.id
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
    pub fn new(id: TagId, name: &str, description: &str) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
        }
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
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn create_note() {
        let id: NoteId = 0.into();
        let name: Rc<str> = "Test".into();
        let content: Rc<str> = "This is a test note...".into();
        let tags: Rc<[Tag]> = vec![].into();
        let note = Note {
            id,
            title: name.clone(),
            content: content.clone(),
            tags: tags.clone(),
        };
        assert_eq!(note, Note::new(id, &name, &content, tags));
    }
    #[test]
    fn get_note() {
        let tags: Rc<[Tag]> = Rc::default();
        let note = Note::new(0.into(), "Test", "This is a test note...", tags);
        assert_eq!(note.title(), "Test");
        assert_eq!(note.content(), "This is a test note...");
    }
}
