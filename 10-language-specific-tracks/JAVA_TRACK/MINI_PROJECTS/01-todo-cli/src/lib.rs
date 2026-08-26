use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

const STORAGE_FILE: &str = "todos.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: String) -> Self {
        Todo {
            id,
            title,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

pub enum TodoFilter {
    All,
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoApp {
    todos: Vec<Todo>,
    next_id: u32,
}

impl TodoApp {
    pub fn new() -> Self {
        TodoApp {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    /// Add a new todo item
    pub fn add_todo(&mut self, title: String) {
        let todo = Todo::new(self.next_id, title);
        self.todos.push(todo);
        self.next_id += 1;
    }

    /// Get todos filtered by status
    pub fn list_todos(&self, filter: TodoFilter) -> Vec<&Todo> {
        match filter {
            TodoFilter::All => self.todos.iter().collect(),
            TodoFilter::Pending => self.todos.iter().filter(|t| !t.completed).collect(),
            TodoFilter::Completed => self.todos.iter().filter(|t| t.completed).collect(),
        }
    }

    /// Mark a todo as complete by index
    pub fn complete_todo(&mut self, index: usize) -> Result<(), String> {
        self.todos
            .get_mut(index)
            .map(|todo| todo.complete())
            .ok_or_else(|| format!("Todo at index {} not found", index))
    }

    /// Delete a todo by index
    pub fn delete_todo(&mut self, index: usize) -> Result<(), String> {
        if index < self.todos.len() {
            self.todos.remove(index);
            Ok(())
        } else {
            Err(format!("Todo at index {} not found", index))
        }
    }

    /// Clear all completed todos
    pub fn clear_completed(&mut self) -> usize {
        let before = self.todos.len();
        self.todos.retain(|todo| !todo.completed);
        before - self.todos.len()
    }

    /// Check if todo list is empty
    pub fn is_empty(&self) -> bool {
        self.todos.is_empty()
    }

    /// Save todos to file
    pub fn save(&self) -> io::Result<()> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(STORAGE_FILE, json)?;
        Ok(())
    }

    /// Load todos from file
    pub fn load() -> io::Result<Self> {
        if !Path::new(STORAGE_FILE).exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Storage file not found",
            ));
        }

        let contents = fs::read_to_string(STORAGE_FILE)?;
        let app: TodoApp = serde_json::from_str(&contents)?;
        Ok(app)
    }
}

impl Default for TodoApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_app() {
        let app = TodoApp::new();
        assert_eq!(app.todos.len(), 0);
        assert_eq!(app.next_id, 1);
    }

    #[test]
    fn test_add_todo() {
        let mut app = TodoApp::new();
        app.add_todo("Buy milk".to_string());

        assert_eq!(app.todos.len(), 1);
        assert_eq!(app.todos[0].title, "Buy milk");
        assert_eq!(app.todos[0].id, 1);
        assert!(!app.todos[0].completed);
    }

    #[test]
    fn test_complete_todo() {
        let mut app = TodoApp::new();
        app.add_todo("Buy milk".to_string());

        let result = app.complete_todo(0);
        assert!(result.is_ok());
        assert!(app.todos[0].completed);
    }

    #[test]
    fn test_complete_invalid_index() {
        let mut app = TodoApp::new();
        let result = app.complete_todo(0);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_todo() {
        let mut app = TodoApp::new();
        app.add_todo("Buy milk".to_string());
        app.add_todo("Walk dog".to_string());

        let result = app.delete_todo(0);
        assert!(result.is_ok());
        assert_eq!(app.todos.len(), 1);
        assert_eq!(app.todos[0].title, "Walk dog");
    }

    #[test]
    fn test_filter_todos() {
        let mut app = TodoApp::new();
        app.add_todo("Buy milk".to_string());
        app.add_todo("Walk dog".to_string());
        app.complete_todo(0).unwrap();

        let all = app.list_todos(TodoFilter::All);
        assert_eq!(all.len(), 2);

        let pending = app.list_todos(TodoFilter::Pending);
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].title, "Walk dog");

        let completed = app.list_todos(TodoFilter::Completed);
        assert_eq!(completed.len(), 1);
        assert_eq!(completed[0].title, "Buy milk");
    }

    #[test]
    fn test_clear_completed() {
        let mut app = TodoApp::new();
        app.add_todo("Buy milk".to_string());
        app.add_todo("Walk dog".to_string());
        app.add_todo("Read book".to_string());
        app.complete_todo(0).unwrap();
        app.complete_todo(2).unwrap();

        let cleared = app.clear_completed();
        assert_eq!(cleared, 2);
        assert_eq!(app.todos.len(), 1);
        assert_eq!(app.todos[0].title, "Walk dog");
    }
}
