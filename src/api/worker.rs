use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

use crate::queue::command::{Command, WorkerId};
use crate::queue::job::{Job, JobId};

pub struct WorkerClient {
    id: WorkerId,
    tx: mpsc::Sender<Command>,
}

impl WorkerClient {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self {
            id: Uuid::new_v4(),
            tx,
        }
    }

    pub async fn pull(&self) -> Option<Job> {
        let (reply_tx, reply_rx) = oneshot::channel();

        let _ = self
            .tx
            .send(Command::Pull {
                worker_id: self.id,
                reply: reply_tx,
            })
            .await;

        reply_rx.await.ok().flatten()
    }

    pub async fn ack(&self, job_id: JobId) {
        let _ = self
            .tx
            .send(Command::Ack {
                job_id,
                worker_id: self.id,
            })
            .await;
    }
}
