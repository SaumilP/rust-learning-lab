use std::sync::Arc;

pub fn share_across_thread() {
    let count = Arc::new(1);
    let handle = std::thread::spawn(move || println!("{count}"));
    handle.join().expect("thread should complete");
}
