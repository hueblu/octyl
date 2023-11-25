use std::{any::Any, sync::atomic::AtomicU64};

use futures::{FutureExt, Stream, StreamExt};
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_stream::StreamMap;
use tokio_util::sync::CancellationToken;

use crate::identifier::Id;

static EVENT_PRODUCER_ID: AtomicU64 = AtomicU64::new(0);

pub type Event = Box<dyn EventTrait>;

impl Clone for Event {
    fn clone(&self) -> Self {
        self._box_clone()
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self._box_eq(other.as_ref())
    }
}

pub trait EventTrait: Any + Send {
    fn _box_clone(&self) -> Box<dyn EventTrait>;
    fn _box_eq(&self, other: &dyn EventTrait) -> bool;

    fn as_any(&self) -> &dyn Any;
}

impl<T> EventTrait for T
where
    T: Clone + PartialEq + Send + 'static,
{
    fn _box_clone(&self) -> Box<dyn EventTrait> {
        Box::new(self.clone())
    }

    fn _box_eq(&self, other: &dyn EventTrait) -> bool {
        if let Some(other) = other.as_any().downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct EventProducer {
    event_rx: mpsc::UnboundedReceiver<Event>,

    join_handle: JoinHandle<()>,
    cancellation_token: CancellationToken,
}

pub struct EventProducerCollection {
    streams: StreamMap<Id, EventProducer>,
}

impl Stream for EventProducer {
    type Item = Event;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.event_rx.poll_recv(cx)
    }
}

impl EventProducer {
    pub fn new<S: Stream<Item = Event> + Unpin + Send + 'static>(mut f: S) -> Self
where {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let cancellation_token = CancellationToken::new();
        let token_clone = cancellation_token.clone();

        let join_handle = tokio::spawn(async move {
            loop {
                let event_future = f.next().fuse();
                let cancellation_future = token_clone.cancelled();

                tokio::select! {
                    event = event_future => {
                        if let Some(event) = event {
                            let _ = event_tx.send(event);
                        }
                    }
                    _ = cancellation_future => {
                    }
                }
            }
        });

        Self {
            event_rx,
            join_handle,
            cancellation_token,
        }
    }
}

impl EventProducerCollection {
    pub fn new() -> Self {
        Self {
            streams: StreamMap::new(),
        }
    }

    pub fn add_event_producer<S: Stream<Item = Event> + Unpin + Send + 'static>(
        &mut self,
        f: S,
    ) -> Id {
        let id = Id::new(&EVENT_PRODUCER_ID);

        self.streams.insert(id, EventProducer::new(f));

        id
    }
}

impl Stream for EventProducerCollection {
    type Item = Event;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.streams.poll_next_unpin(cx).map(|f| f.unzip().1)
    }
}
