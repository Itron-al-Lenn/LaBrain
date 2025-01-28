# How should this work?

So... What do I need to know for each note?

- Time of creation
- Time of last update
- Time of last compilation
- Title
- Content
- Description

# How should I store the text?

I'm pretty sure, that I want to render the text using LaTeX... But how should I store the content themself?

- Latex
- MD
- txt?

-> LaTeX. The Content itself is stored in the DB

# What DB?

Where / How to store this stuff?

- SQLite -> Would be easy
- PostgreSQL for syncing over multiple devices? -> Maybe later

# Note types
- Fleeting Note
- Idea Note / Refined Note
- Literature Note
- Meta Notes

## Calender Notes
- Event Note
- Repeating Note / Schedule Note

# RANDOM IDEAS:

- Specify packages in config file (general and note type specific)
- Calender Note with calender features
