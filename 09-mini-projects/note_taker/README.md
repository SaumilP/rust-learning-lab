# Note-Taking Application

## Overview

A flexible note management system supporting multiple note types. Demonstrates polymorphism, complex data structures, and serialization.

## Concepts Learned

- **Trait Objects**: Different note types (text, list, code)
- **Composition**: Combining multiple traits
- **Ownership Patterns**: Shared note access
- **Lifetimes**: References in complex structures
- **Serialization**: serde for JSON/YAML export
- **Collections**: Organizing notes by category
- **Algorithms**: Full-text search implementation

## Features

1. **Multiple Note Types**:
   - Text notes with formatting
   - Checkable lists
   - Code snippets with syntax highlighting
   - Quick notes

2. **Organization**:
   - Tagging system
   - Categorization
   - Timestamps
   - Access patterns

3. **Operations**:
   - Create, read, update, delete
   - Search and filter
   - Export formats (JSON, YAML, Markdown)
   - Backup and restore

## Data Structures

```rust
pub trait Note {
    fn title(&self) -> &str;
    fn content(&self) -> String;
    fn tags(&self) -> &[String];
    fn update_content(&mut self, content: &str);
}

pub struct TextNote {
    title: String,
    content: String,
    tags: Vec<String>,
    created_at: DateTime,
}

pub struct ListNote {
    title: String,
    items: Vec<CheckItem>,
    tags: Vec<String>,
}

pub struct CodeNote {
    title: String,
    language: String,
    content: String,
    tags: Vec<String>,
}

pub struct NoteBook {
    notes: Vec<Box<dyn Note>>,
    categories: HashMap<String, Vec<usize>>,
}
```

## Commands

```bash
# Create notes
cargo run -- new text "Note Title" --tag work

# List notes
cargo run -- list
cargo run -- list --tag work
cargo run -- list --category projects

# Search
cargo run -- search "keyword"

# Export
cargo run -- export all --format json
cargo run -- export --tag work --format yaml

# Organize
cargo run -- tag 1 personal
cargo run -- untag 1 work
```

## Implementation Details

### Module Organization
```
src/
├── main.rs              # CLI entry point
├── note_trait.rs        # Note trait definition
├── notes/
│   ├── text.rs         # TextNote implementation
│   ├── list.rs         # ListNote implementation
│   └── code.rs         # CodeNote implementation
├── notebook.rs          # NoteBook management
├── search.rs           # Search functionality
└── export.rs           # Export formats
```

### Key Functions

```rust
pub trait Note {
    fn title(&self) -> &str;
    fn content(&self) -> String;
    fn search(&self, query: &str) -> bool;
    fn export(&self, format: Format) -> String;
}

pub fn create_note(note_type: NoteType, title: &str) -> Box<dyn Note>
pub fn search_notes(notebook: &NoteBook, query: &str) -> Vec<&dyn Note>
pub fn organize_by_tag(notebook: &NoteBook) -> HashMap<String, Vec<&dyn Note>>
```

### Polymorphism Example

```rust
pub struct NoteBook {
    notes: Vec<Box<dyn Note>>,
}

// Can store different note types together
notebook.add_note(Box::new(TextNote::new(...)));
notebook.add_note(Box::new(ListNote::new(...)));
notebook.add_note(Box::new(CodeNote::new(...)));
```

## Testing

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration_tests

# Search performance
cargo test search -- --nocapture

# Serialization
cargo test export
```

## Extension Possibilities

1. **Database Persistence**: SQLite backend
2. **Encryption**: Secure sensitive notes
3. **Collaboration**: Share notes with others
4. **Web Interface**: Browser-based editor
5. **Sync**: Cloud synchronization
6. **AI Integration**: Automatic tagging and summarization

## Learning Outcomes

After completing this project:
- ✓ Trait object design patterns
- ✓ Complex ownership structures
- ✓ Serialization/deserialization
- ✓ Search algorithm implementation
- ✓ Export format handling

## Common Patterns Used

### Trait Object Storage
```rust
let notes: Vec<Box<dyn Note>> = vec![
    Box::new(text_note),
    Box::new(list_note),
    Box::new(code_note),
];

for note in notes {
    println!("{}", note.title());
}
```

### Conditional Trait Implementation
```rust
impl Note for TextNote {
    fn search(&self, query: &str) -> bool {
        self.content.to_lowercase().contains(&query.to_lowercase())
    }
}
```

### Export Pattern
```rust
pub fn export_notes(notes: &[Box<dyn Note>], format: Format) -> String {
    match format {
        Format::Json => export_json(notes),
        Format::Yaml => export_yaml(notes),
        Format::Markdown => export_markdown(notes),
    }
}
```

## Estimated Development Time

- Beginner: 8-10 hours
- Intermediate: 4-5 hours
- Experienced Rust: 2-3 hours

## Related Modules

- Module 06: Traits and trait objects
- Module 06: Ownership patterns
- Module 08: Factory pattern for note creation

