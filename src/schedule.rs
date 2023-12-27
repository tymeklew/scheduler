use std::{
    future::Future,
    time::{Duration, Instant},
};

use tokio::sync::mpsc::{channel, Sender};
use uuid::Uuid;

use crate::job::Job;

pub enum Message {
    AddJob(Job),
    TerminateJob(Uuid),
    Stop,
}

pub struct Schedule {
    sender: Sender<Message>,
}

impl Schedule {
    pub fn start() -> Schedule {
        let (sender, mut rx) = channel::<Message>(100);

        tokio::spawn(async move {
            let mut jobs: Vec<Job> = Vec::new();

            loop {
                if let Ok(msg) = rx.try_recv() {
                    match msg {
                        Message::AddJob(job) => jobs.push(job),
                        Message::Stop => break,
                        Message::TerminateJob(id) => {
                            if let Some(index) = jobs.iter().position(|f| f.id == id) {
                                jobs.swap_remove(index);
                            }
                        }
                    }
                }

                let now = Instant::now();

                for job in jobs.iter_mut() {
                    if (job.last_ran_at + job.interval) < now {
                        job.last_ran_at = now;
                    }
                }
            }
        });

        Schedule { sender }
    }

    pub async fn send(&self, message: Message) {
        self.sender.send(message).await.unwrap();
    }

    pub async fn add_job<Fut>(&self, interval: Duration, callback: impl FnOnce() -> Fut) -> Uuid
    where
        Fut: Future<Output = ()>,
    {
        let id = Uuid::new_v4();

        self.send(Message::AddJob(Job {
            id,
            interval,
            last_ran_at: Instant::now(),
        }))
        .await;

        id
    }
}
