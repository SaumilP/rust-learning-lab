//! Memento: capture state so it can be restored without exposing internals.

#[derive(Clone)]
struct EditorMemento {
    text: String,
}

struct Editor {
    text: String,
}

impl Editor {
    fn new() -> Self {
        Self {
            text: String::new(),
        }
    }
    fn write(&mut self, text: &str) {
        self.text.push_str(text);
    }
    fn save(&self) -> EditorMemento {
        EditorMemento {
            text: self.text.clone(),
        }
    }
    fn restore(&mut self, memento: EditorMemento) {
        self.text = memento.text;
    }
    fn text(&self) -> &str {
        &self.text
    }
}

fn main() {
    let mut editor = Editor::new();
    editor.write("Version one");
    let saved = editor.save();
    editor.write(" with an unwanted edit");
    editor.restore(saved);
    println!("{}", editor.text());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn restores_a_previous_snapshot() {
        let mut editor = Editor::new();
        editor.write("saved");
        let snapshot = editor.save();
        editor.write(" changed");
        editor.restore(snapshot);
        assert_eq!(editor.text(), "saved");
    }
}
