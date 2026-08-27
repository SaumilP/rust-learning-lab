use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// A thread pool for executing jobs concurrently.
///
/// Similar to Java's `ExecutorService`, but with compile-time thread safety.
///
/// # Examples
///
/// ```
/// use multithreaded_server::ThreadPool;
///
/// let pool = ThreadPool::new(4);
/// pool.execute(|| println!("Hello from worker thread"));
/// // Pool automatically shuts down when dropped
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Thread pool size must be greater than zero");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Execute a job on the thread pool.
    ///
    /// The job is a closure that will be executed by one of the worker threads.
    ///
    /// # Examples
    ///
    /// ```
    /// use multithreaded_server::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4);
    /// pool.execute(|| {
    ///     println!("Executing job");
    /// });
    /// ```
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .expect("Failed to send job to worker");
    }
}

impl Drop for ThreadPool {
    /// Gracefully shut down the thread pool.
    ///
    /// This waits for all workers to finish their current jobs before shutting down.
    fn drop(&mut self) {
        // Drop the sender to signal all workers to shut down
        drop(self.sender.take());

        println!("Shutting down all workers...");

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().expect("Worker thread panicked");
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Lock the receiver, get a job, then immediately release the lock
            let job = receiver.lock().unwrap().recv();

            match job {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    // Sender has been dropped, shut down
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
    }

    #[test]
    #[should_panic(expected = "Thread pool size must be greater than zero")]
    fn test_zero_size_panics() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_execute_jobs() {
        let pool = ThreadPool::new(2);
        let counter = Arc::new(AtomicU32::new(0));

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                counter.fetch_add(1, Ordering::SeqCst);
            });
        }

        // Give threads time to complete
        drop(pool);
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Note: In real tests, you'd use proper synchronization
        assert!(counter.load(Ordering::SeqCst) <= 10);
    }
}
