use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::queue::command::Command;
use crate::queue::job::Job;


struct Consumer {
    rx: oneshot::Receiver<Command>,
}

impl Consumer {
    fn new(rx: oneshot::Receiver<Command>) -> Self {
        Self { rx }
    }

    fn pull(rx)
}