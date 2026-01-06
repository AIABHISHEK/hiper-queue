use std::collections::{HashMap, VecDeque};

use super::command::{Command, WorkerId};
use super::job::{Job, JobId};


struct InProgressJob {
    job: Job,
    worker_id: WorkerId,
    started_at: Instant,
}

pub struct Engine {
    queue: VecDeque<Job>,
    progress_queue: HashMap<JobId, InProgressJob>
}
