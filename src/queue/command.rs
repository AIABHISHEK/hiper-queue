use tokio::sync::oneshot;
use uuid::Uuid;

use super::job::{Job, JobId};

pub type WorkerId = Uuid;

pub enum Command {
    Push {
        job: Job,
    },
    Pull {
        worker_id: WorkerId,
        reply: oneshot::Sender<Option<Job>>,
    },
    Ack {
        job_id: JobId,
        worker_id: WorkerId,
    },
}
