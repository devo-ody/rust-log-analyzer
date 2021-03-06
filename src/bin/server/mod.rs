use crate::rla;

pub use self::service::RlaService;
pub use self::worker::Worker;

mod service;
mod worker;

pub enum QueueItem {
    GitHubStatus(rla::github::CommitStatusEvent),
    GitHubCheckRun(rla::github::CheckRunEvent),
}
