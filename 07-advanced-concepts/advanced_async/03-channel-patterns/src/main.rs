use tokio::sync::{broadcast, mpsc, oneshot, watch};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("📡 Channel Patterns Demo\n");

    demo_mpsc().await;
    demo_broadcast().await;
    demo_watch().await;
    demo_oneshot().await;

    println!("\n✅ All channel demos completed!");
}

async fn demo_mpsc() {
    println!("=== 1. MPSC (Multiple Producer, Single Consumer) ===");

    let (tx, mut rx) = mpsc::channel(32);

    // Spawn multiple producers
    for i in 0..5 {
        let tx = tx.clone();
        tokio::spawn(async move {
            sleep(Duration::from_millis(i * 50)).await;
            tx.send(format!("Message from producer {}", i))
                .await
                .unwrap();
        });
    }

    drop(tx); // Close channel when all senders are done

    // Single consumer receives all messages
    while let Some(msg) = rx.recv().await {
        println!("   📥 Received: {}", msg);
    }

    println!();
}

async fn demo_broadcast() {
    println!("=== 2. Broadcast (Multiple Consumers) ===");

    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();
    let mut rx3 = tx.subscribe();

    // Spawn consumers
    tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("   📻 Consumer 1 got: {}", msg);
        }
    });

    tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("   📻 Consumer 2 got: {}", msg);
        }
    });

    tokio::spawn(async move {
        while let Ok(msg) = rx3.recv().await {
            println!("   📻 Consumer 3 got: {}", msg);
        }
    });

    // Producer sends messages to all consumers
    for i in 0..3 {
        tx.send(format!("Broadcast message {}", i)).unwrap();
        sleep(Duration::from_millis(100)).await;
    }

    sleep(Duration::from_millis(200)).await;
    println!();
}

async fn demo_watch() {
    println!("=== 3. Watch (Latest Value) ===");

    let (tx, mut rx1) = watch::channel("initial");
    let mut rx2 = tx.subscribe();

    // Spawn watchers
    tokio::spawn(async move {
        while rx1.changed().await.is_ok() {
            println!("   👁️  Watcher 1 sees: {}", *rx1.borrow());
        }
    });

    tokio::spawn(async move {
        while rx2.changed().await.is_ok() {
            println!("   👁️  Watcher 2 sees: {}", *rx2.borrow());
        }
    });

    // Update state multiple times
    sleep(Duration::from_millis(50)).await;
    tx.send("first update").unwrap();

    sleep(Duration::from_millis(50)).await;
    tx.send("second update").unwrap();

    sleep(Duration::from_millis(50)).await;
    tx.send("final value").unwrap();

    sleep(Duration::from_millis(100)).await;
    println!();
}

async fn demo_oneshot() {
    println!("=== 4. Oneshot (Request-Response) ===");

    // Request-response pattern
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        println!("   🔄 Processing request...");
        sleep(Duration::from_millis(100)).await;
        tx.send("Response data").unwrap();
    });

    match rx.await {
        Ok(response) => println!("   ✉️  Got response: {}", response),
        Err(_) => println!("   ❌ Sender dropped"),
    }

    println!();

    // Multiple request-response
    let mut handles = vec![];

    for i in 0..3 {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            sleep(Duration::from_millis(i * 50)).await;
            tx.send(i * 10).unwrap();
        });

        handles.push(rx);
    }

    for (idx, rx) in handles.into_iter().enumerate() {
        if let Ok(value) = rx.await {
            println!("   📬 Request {} got: {}", idx, value);
        }
    }

    println!();
}
