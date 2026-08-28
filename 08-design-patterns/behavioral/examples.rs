// Behavioral Patterns Examples

use std::cell::RefCell;
use std::rc::Rc;

// ============= Observer Pattern Example =============
pub trait EventObserver {
    fn on_event(&self, event: &str);
}

pub struct EventEmitter {
    observers: RefCell<Vec<Rc<dyn EventObserver>>>,
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            observers: RefCell::new(Vec::new()),
        }
    }

    pub fn subscribe(&self, observer: Rc<dyn EventObserver>) {
        self.observers.borrow_mut().push(observer);
    }

    pub fn emit(&self, event: &str) {
        let observers = self.observers.borrow();
        for observer in observers.iter() {
            observer.on_event(event);
        }
    }
}

pub struct ConsoleObserver {
    name: String,
}

impl ConsoleObserver {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl EventObserver for ConsoleObserver {
    fn on_event(&self, event: &str) {
        println!("{} received event: {}", self.name, event);
    }
}

pub struct LogFileObserver;

impl EventObserver for LogFileObserver {
    fn on_event(&self, event: &str) {
        println!("[LOG] Event recorded: {}", event);
    }
}

// ============= Strategy Pattern Example =============
pub trait SortingStrategy {
    fn sort(&self, data: &mut Vec<i32>);
}

pub struct BubbleSort;

impl SortingStrategy for BubbleSort {
    fn sort(&self, data: &mut Vec<i32>) {
        println!("Sorting with Bubble Sort");
        let n = data.len();
        for i in 0..n {
            for j in 0..n - i - 1 {
                if data[j] > data[j + 1] {
                    data.swap(j, j + 1);
                }
            }
        }
    }
}

pub struct QuickSort;

impl SortingStrategy for QuickSort {
    fn sort(&self, data: &mut Vec<i32>) {
        println!("Sorting with Quick Sort");
        data.sort();
    }
}

pub struct Sorter {
    strategy: Box<dyn SortingStrategy>,
}

impl Sorter {
    pub fn new(strategy: Box<dyn SortingStrategy>) -> Self {
        Self { strategy }
    }

    pub fn sort(&self, data: &mut Vec<i32>) {
        self.strategy.sort(data);
    }

    pub fn set_strategy(&mut self, strategy: Box<dyn SortingStrategy>) {
        self.strategy = strategy;
    }
}

// ============= Command Pattern Example =============
pub trait Action {
    fn execute(&mut self);
    fn undo(&mut self);
}

pub struct TextBuffer {
    content: String,
}

impl TextBuffer {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn append(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn delete_last(&mut self, count: usize) {
        for _ in 0..count {
            self.content.pop();
        }
    }

    pub fn display(&self) {
        println!("Buffer: '{}'", self.content);
    }
}

pub struct AppendCommand {
    buffer: TextBuffer,
    text: String,
}

impl AppendCommand {
    pub fn new(buffer: TextBuffer, text: &str) -> Self {
        Self {
            buffer,
            text: text.to_string(),
        }
    }
}

impl Action for AppendCommand {
    fn execute(&mut self) {
        self.buffer.append(&self.text);
    }

    fn undo(&mut self) {
        for _ in 0..self.text.len() {
            self.buffer.delete_last(1);
        }
    }
}

// ============= State Pattern Example =============
pub trait DoorState {
    fn open(&self) -> Box<dyn DoorState>;
    fn close(&self) -> Box<dyn DoorState>;
    fn description(&self) -> &'static str;
}

pub struct OpenDoor;

impl DoorState for OpenDoor {
    fn open(&self) -> Box<dyn DoorState> {
        println!("Door is already open");
        Box::new(OpenDoor)
    }

    fn close(&self) -> Box<dyn DoorState> {
        println!("Closing door...");
        Box::new(ClosedDoor)
    }

    fn description(&self) -> &'static str {
        "Door is open"
    }
}

pub struct ClosedDoor;

impl DoorState for ClosedDoor {
    fn open(&self) -> Box<dyn DoorState> {
        println!("Opening door...");
        Box::new(OpenDoor)
    }

    fn close(&self) -> Box<dyn DoorState> {
        println!("Door is already closed");
        Box::new(ClosedDoor)
    }

    fn description(&self) -> &'static str {
        "Door is closed"
    }
}

pub struct Door {
    state: Box<dyn DoorState>,
}

impl Door {
    pub fn new() -> Self {
        Self {
            state: Box::new(ClosedDoor),
        }
    }

    pub fn open(&mut self) {
        self.state = self.state.open();
    }

    pub fn close(&mut self) {
        self.state = self.state.close();
    }

    pub fn status(&self) {
        println!("Status: {}", self.state.description());
    }
}

// ============= Usage Example =============
pub fn example_behavioral_patterns() {
    println!("=== Observer Pattern ===");
    let emitter = Rc::new(EventEmitter::new());
    let console_obs = Rc::new(ConsoleObserver::new("Observer1"));
    let log_obs = Rc::new(LogFileObserver);

    emitter.subscribe(console_obs);
    emitter.subscribe(log_obs);
    emitter.emit("User logged in");

    println!("\n=== Strategy Pattern ===");
    let mut data = vec![5, 2, 8, 1, 9];
    let mut sorter = Sorter::new(Box::new(BubbleSort));
    sorter.sort(&mut data);
    println!("Sorted (Bubble): {:?}", data);

    let mut data2 = vec![5, 2, 8, 1, 9];
    sorter.set_strategy(Box::new(QuickSort));
    sorter.sort(&mut data2);
    println!("Sorted (Quick): {:?}", data2);

    println!("\n=== Command Pattern ===");
    let buffer = TextBuffer::new();
    let mut cmd = AppendCommand::new(buffer, "Hello");
    cmd.execute();
    cmd.buffer.display();

    println!("\n=== State Pattern ===");
    let mut door = Door::new();
    door.status();
    door.open();
    door.status();
    door.close();
    door.status();
}

fn main() {
    example_behavioral_patterns();
}
