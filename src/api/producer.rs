use tokio::sync::mpsc;

use crate::queue::command::Command;
use crate::queue::job::Job;

pub struct Producer {
    tx: mpsc::Sender<Command>,
}

impl Producer {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self { tx }
    }

    pub async fn push(&self, job: Job) {
        let _ = self.tx.send(Command::Push { job }).await;
    }
}
