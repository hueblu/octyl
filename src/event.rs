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
    handlers: Vec<task::JoinHandle<Result<ThreadStatus>>>,

    pub receiver: broadcast::Receiver<Event>,
}

enum ThreadStatus {
    Failed,
    Completed,
}

impl EventHandler {
    pub fn new() -> Result<Self> {
        log::debug!("Initializing event handler");

        let cancel = CancellationToken::new();
        let (tx, rx) = broadcast::channel(MAX_EVENTS);
        let mut handlers = Vec::new();

        handlers.push(spawn_terminal_thread(cancel.clone(), tx.clone()));
        handlers.push(spawn_tick_thread(cancel.clone(), tx.clone()));

        Ok(Self {
            cancel,
            handlers,
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

fn spawn_terminal_thread(
    cancel: CancellationToken,
    sender: broadcast::Sender<Event>,
) -> task::JoinHandle<Result<ThreadStatus>> {
    log::trace!("Spawning terminal thread");

    let mut eventstream = EventStream::new();

    task::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel.cancelled() => return Ok(ThreadStatus::Completed),
                event = eventstream.next() => {
                    if let Some(event) = event {
                        log::debug!("terminal event sent: {:?}", event);
                        sender.send(Event::Terminal(event.expect("something bad happened crossterm and thread idk")))?;
                    }
                }
            }
        }
    })
}

fn spawn_tick_thread(
    cancel: CancellationToken,
    sender: broadcast::Sender<Event>,
) -> task::JoinHandle<Result<ThreadStatus>> {
    log::trace!("Spawning tick thread");

    let mut interval = time::interval(Duration::from_secs_f64(TICK_RATE / 60.));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    task::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel.cancelled() => return Ok(ThreadStatus::Completed),
                _ = interval.tick() => {
                    log::trace!("tick event sent");
                    sender.send(Event::Tick)?;
                }
            }
        }
    })
}
