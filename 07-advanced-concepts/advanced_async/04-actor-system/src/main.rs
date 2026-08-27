use tokio::sync::{mpsc, oneshot};
use tokio::time::{sleep, Duration};

// Message types for Counter actor
enum CounterMessage {
    Increment,
    Decrement,
    GetCount { respond_to: oneshot::Sender<i64> },
    Reset,
}

// Counter Actor - maintains state and processes messages
struct CounterActor {
    count: i64,
    receiver: mpsc::Receiver<CounterMessage>,
}

impl CounterActor {
    fn new(receiver: mpsc::Receiver<CounterMessage>) -> Self {
        CounterActor {
            count: 0,
            receiver,
        }
    }

    async fn run(mut self) {
        println!("🎬 Counter actor started");

        while let Some(msg) = self.receiver.recv().await {
            match msg {
                CounterMessage::Increment => {
                    self.count += 1;
                    println!("   [Counter] Incremented: {}", self.count);
                }
                CounterMessage::Decrement => {
                    self.count -= 1;
                    println!("   [Counter] Decremented: {}", self.count);
                }
                CounterMessage::GetCount { respond_to } => {
                    let _ = respond_to.send(self.count);
                    println!("   [Counter] Sent count: {}", self.count);
                }
                CounterMessage::Reset => {
                    self.count = 0;
                    println!("   [Counter] Reset to 0");
                }
            }
        }

        println!("🛑 Counter actor stopped");
    }
}

// Handle for communicating with the actor
#[derive(Clone)]
struct CounterHandle {
    sender: mpsc::Sender<CounterMessage>,
}

impl CounterHandle {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel(32);
        let actor = CounterActor::new(receiver);

        // Spawn actor
        tokio::spawn(actor.run());

        CounterHandle { sender }
    }

    async fn increment(&self) {
        let _ = self.sender.send(CounterMessage::Increment).await;
    }

    async fn decrement(&self) {
        let _ = self.sender.send(CounterMessage::Decrement).await;
    }

    async fn get_count(&self) -> i64 {
        let (tx, rx) = oneshot::channel();
        let _ = self.sender.send(CounterMessage::GetCount { respond_to: tx }).await;
        rx.await.unwrap_or(0)
    }

    async fn reset(&self) {
        let _ = self.sender.send(CounterMessage::Reset).await;
    }
}

// Worker Actor - processes tasks
enum WorkerMessage {
    ProcessTask { task_id: u64, data: String },
}

struct WorkerActor {
    id: u64,
    receiver: mpsc::Receiver<WorkerMessage>,
}

impl WorkerActor {
    fn new(id: u64, receiver: mpsc::Receiver<WorkerMessage>) -> Self {
        WorkerActor { id, receiver }
    }

    async fn run(mut self) {
        println!("👷 Worker {} started", self.id);

        while let Some(msg) = self.receiver.recv().await {
            match msg {
                WorkerMessage::ProcessTask { task_id, data } => {
                    println!(
                        "   [Worker {}] Processing task {}: {}",
                        self.id, task_id, data
                    );
                    // Simulate work
                    sleep(Duration::from_millis(100)).await;
                    println!("   [Worker {}] Completed task {}", self.id, task_id);
                }
            }
        }

        println!("🛑 Worker {} stopped", self.id);
    }
}

#[derive(Clone)]
struct WorkerHandle {
    sender: mpsc::Sender<WorkerMessage>,
}

impl WorkerHandle {
    fn new(id: u64) -> Self {
        let (sender, receiver) = mpsc::channel(32);
        let actor = WorkerActor::new(id, receiver);
        tokio::spawn(actor.run());

        WorkerHandle { sender }
    }

    async fn process_task(&self, task_id: u64, data: String) {
        let _ = self
            .sender
            .send(WorkerMessage::ProcessTask { task_id, data })
            .await;
    }
}

// Worker Pool - distributes work across workers
struct WorkerPool {
    workers: Vec<WorkerHandle>,
    next_worker: std::sync::atomic::AtomicUsize,
}

impl WorkerPool {
    fn new(num_workers: usize) -> Self {
        let workers = (0..num_workers)
            .map(|id| WorkerHandle::new(id as u64))
            .collect();

        WorkerPool {
            workers,
            next_worker: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    async fn submit_task(&self, task_id: u64, data: String) {
        use std::sync::atomic::Ordering;

        // Round-robin task distribution
        let worker_idx = self.next_worker.fetch_add(1, Ordering::Relaxed) % self.workers.len();
        self.workers[worker_idx].process_task(task_id, data).await;
    }
}

#[tokio::main]
async fn main() {
    println!("🎭 Actor System Demo\n");

    // Demo 1: Counter Actor
    println!("=== Demo 1: Counter Actor ===");
    let counter = CounterHandle::new();

    counter.increment().await;
    counter.increment().await;
    counter.increment().await;

    let count = counter.get_count().await;
    println!("📊 Current count: {}\n", count);

    sleep(Duration::from_millis(100)).await;

    // Demo 2: Multiple concurrent clients
    println!("=== Demo 2: Concurrent Clients ===");
    let counter2 = CounterHandle::new();

    let handles: Vec<_> = (0..5)
        .map(|i| {
            let c = counter2.clone();
            tokio::spawn(async move {
                for _ in 0..3 {
                    c.increment().await;
                    sleep(Duration::from_millis(50)).await;
                }
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }

    let final_count = counter2.get_count().await;
    println!("📊 Final count: {}\n", final_count);

    sleep(Duration::from_millis(100)).await;

    // Demo 3: Worker Pool
    println!("=== Demo 3: Worker Pool ===");
    let pool = WorkerPool::new(3);

    // Submit tasks
    for i in 0..9 {
        pool.submit_task(i, format!("Task data {}", i)).await;
    }

    // Wait for tasks to complete
    sleep(Duration::from_secs(1)).await;

    println!("\n✅ All demos completed!");
}
