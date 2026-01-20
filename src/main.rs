mod api;
mod queue;


use tokio::sync::mpsc;
use tracing_subscriber::fmt;

use api::{producer::Producer, worker::WorkerClient};
use queue::engine::Engine;
use queue::job::Job;

#[tokio::main]
async fn main() {
    fmt().with_target(false).init();

    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let engine = Engine::new();
        let mut engine = Engine::new();
        engine.run(rx).await;
    });

    let producer = Producer::new(tx.clone());
    let worker = WorkerClient::new(tx.clone());

    producer
        .push(Job::new(b"hello world".to_vec(), 3))
        .await;

    if let Some(job) = worker.pull().await {
        println!("Worker got job {:?}", job.id);
        worker.ack(job.id).await;
    }
}
