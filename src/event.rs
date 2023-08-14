use std::time::Duration;

use anyhow::Result;
use crossterm::event::{Event as TerminalEvent, EventStream};
use futures::StreamExt;
use tokio::{sync::broadcast, task, time};
use tokio_util::sync::CancellationToken;

const MAX_EVENTS: usize = 100;
// ticks per second
const TICK_RATE: f64 = 5.;

#[derive(Clone, Debug)]
pub enum Event {
    CtrlC,
    Tick,
    Terminal(TerminalEvent),
}

pub struct EventHandler {
    cancel: CancellationToken,
    handler: task::JoinHandle<Result<ThreadStatus>>,

    pub receiver: broadcast::Receiver<Event>,
}

enum ThreadStatus {
    Failed,
    Completed,
}

impl EventHandler {
    pub fn new() -> Result<Self> {
        let cancel = CancellationToken::new();
        let (tx, rx) = broadcast::channel(MAX_EVENTS);

        let handler = spawn_handler_thread(cancel.clone(), tx.clone());

        Ok(Self {
            cancel,
            handler,
            receiver: rx,
        })
    }
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        self.cancel.cancel();

        // wait for the threads to complete
    }
}

fn spawn_handler_thread(
    cancel: CancellationToken,
    sender: broadcast::Sender<Event>,
) -> task::JoinHandle<Result<ThreadStatus>> {
    let mut eventstream = EventStream::new();
    let mut interval = time::interval(Duration::from_secs_f64(TICK_RATE / 60.));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    task::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel.cancelled() => return Ok(ThreadStatus::Completed),
                event = eventstream.next() => {
                    if let Some(event) = event {
                        sender.send(Event::Terminal(event.expect("something bad happened crossterm and thread idk")))?;
                    }
                }
                _ = interval.tick() => {
                    sender.send(Event::Tick)?;
                }
            }
        }
    })
}
