use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use tokio::sync::mpsc;
use tracing::{info, warn};

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

impl Engine {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            progress_queue: HashMap::new()
        }
    }

    async fn run(&mut self, mut rx: mpsc::Receiver<Command>) {
        info!("Engine started");
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::push(job) => {
                    info!("Pushed job {}", job.id);
                    self.queue.push_back(job);
                }
                Command::pull(worker_id, reply) => {
                    let job = self.queue.pop_front();

                    if let Some(job) = job {
                        let job_id = job.id;

                        self.in_progress.insert(
                            job_id,
                            InProgressJob {
                                job,
                                worker_id,
                                started_at: Instant::now(),
                            },
                        );

                        let job = self
                            .in_progress
                            .get(&job_id)
                            .map(|entry| entry.job.clone());

                        let _ = reply.send(job);
                    } else {
                            let _ = reply.send(None);
                        }
                    }
                    
                }
            }
        }
}