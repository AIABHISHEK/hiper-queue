use uuid::Uuid;

pub type JobId = Uuid;

#[derive(Debug, Clone)]
pub struct Job {
    pub id: JobId,
    pub payload: Vec<u8>,
    pub retries: u32,
    pub max_retries: u8,
}

impl Job {
    pub fn new(payload: Vec<u8>, max_retries: u8) -> Self {
        //validation for max_retries
        //TODO
        Self {
            id: Uuid::new_v4(),
            payload,
            retries: 0,
            max_retries,
        }
    }
}
