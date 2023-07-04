use crossterm::event::Event as TerminalEvent;
use tokio::sync::{broadcast, mpsc};
use tokio::task::{self, JoinHandle};
use tokio_stream::StreamExt;

const TICK_RATE: tokio::time::Duration = tokio::time::Duration::from_millis(100);

#[derive(Debug, PartialEq)]
pub enum Event {
    Tick,
    Terminal(TerminalEvent),
    Quit,
}

pub struct EventHandler {
    pub event_rx: mpsc::Receiver<Event>,
    kill_tx: broadcast::Sender<()>,
}

impl EventHandler {
    pub fn new() -> Self {
        let (kill_tx, _) = tokio::sync::broadcast::channel(1);
        let (event_tx, event_rx) = tokio::sync::mpsc::channel(100);

        spawn_tick_task(event_tx.clone(), kill_tx.subscribe());
        spawn_term_task(event_tx.clone(), kill_tx.subscribe());

        Self { event_rx, kill_tx }
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.event_rx.recv().await
    }
}

fn spawn_tick_task(
    event_tx: mpsc::Sender<Event>,
    mut kill_rx: broadcast::Receiver<()>,
) -> JoinHandle<()> {
    let mut interval = tokio::time::interval(TICK_RATE);

    task::spawn(async move {
        loop {
            tokio::select! {
                biased;
                _ = kill_rx.recv() => {
                    tracing::info!("tick task received kill signal");
                    return;
                }
                _ = interval.tick() => {
                    tracing::trace!("tick");
                    event_tx.send(Event::Tick).await.unwrap();
                }
            }
        }
    })
}

fn spawn_term_task(
    event_tx: mpsc::Sender<Event>,
    mut kill_rx: broadcast::Receiver<()>,
) -> JoinHandle<()> {
    let mut event_stream = crossterm::event::EventStream::new();

    task::spawn(async move {
        loop {
            tokio::select! {
                biased;
                _ = kill_rx.recv() => {
                    tracing::info!("term task received kill signal");
                    return;
                }
                event = event_stream.next() => {
                    tracing::debug!("terminal event received: {:?}", event);

                    match event {
                        Some(Ok(event)) => {
                            event_tx.send(Event::Terminal(event)).await.unwrap()
                        },
                        _ => {
                            tracing::error!("terminal event stream closed");
                            return;
                        }
                    }
                }
            }
        }
    })
}

impl Drop for EventHandler {
    fn drop(&mut self) {
        self.kill_tx.send(()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use tokio::sync::mpsc::error::TryRecvError;

    #[tokio::test]
    async fn tick_task() -> Result<()> {
        let (event_tx, mut event_rx) = tokio::sync::mpsc::channel(100);
        let (kill_tx, kill_rx) = tokio::sync::broadcast::channel(1);

        let _ = spawn_tick_task(event_tx, kill_rx);

        assert_eq!(event_rx.recv().await.unwrap(), Event::Tick);

        kill_tx.send(())?;

        loop {
            match event_rx.try_recv() {
                Ok(event) => assert_eq!(event, Event::Tick),
                Err(TryRecvError::Empty) => break,
                e => panic!("unexpected event {:?}", e),
            }
        }

        Ok(())
    }
}
