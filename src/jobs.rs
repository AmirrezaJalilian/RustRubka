use std::time::Duration;
use tokio::time::sleep;

pub struct Job {
    _delay: u64,
}

impl Job {
    pub fn new(delay_seconds: u64, callback: Box<dyn Fn() + Send + Sync>) -> Self {
        let delay = delay_seconds;
        let job = Job { _delay: delay };
        
        tokio::spawn(async move {
            sleep(Duration::from_secs(delay)).await;
            callback();
        });
        
        job
    }
}

