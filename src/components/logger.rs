use std::{collections::HashMap, io::Write, sync::Arc};

use anyhow::Result;
use circular_buffer::CircularBuffer;
use lazy_static::lazy_static;
use ratatui::widgets::{List, ListItem};
use std::sync::Mutex;

use crate::{
    component::{Component, ComponentState},
    event::Event,
};

lazy_static! {
    pub static ref LOGS: Arc<Mutex<CircularBuffer<25, String>>> =
        Arc::new(Mutex::new(CircularBuffer::new()));
}

pub struct GlobalLogWriter;

impl Write for GlobalLogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        LOGS.lock()
            .unwrap()
            .push_front(String::from_utf8(buf.into()).unwrap());
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub fn logger_component() -> Component {
    Component::new(
        Box::new(|_| Ok(())),
        Box::new(|| HashMap::new()),
        Vec::new(),
        Box::new(|_| None),
        Box::new(|_, _, _| None),
        Box::new(|state, frame, area| {
            let list = List::new(
                LOGS.lock()
                    .unwrap()
                    .iter()
                    .map(|f| ListItem::new(f.clone()))
                    .collect::<Vec<ListItem>>(),
            );

            frame.render_widget(list, area);
        }),
    )
}
