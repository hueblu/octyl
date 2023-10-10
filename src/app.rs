use anyhow::Result;
use futures::Future;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::component::{ComponentTree, Components};
use crate::message::{AppMessage, Message};

pub type BoxFuture<T> = futures::future::BoxFuture<'static, T>;
pub type BoxStream<T> = futures::stream::BoxStream<'static, T>;

pub type BoxMessage = Box<dyn Message>;
pub type BoxMessageFuture = BoxFuture<BoxMessage>;
pub type BoxMessageStream = BoxStream<BoxMessage>;

pub struct App {
    messages: UnboundedReceiverStream<BoxMessage>,

    layout: ComponentTree,
    components: Components,

    suspended: bool,
    should_quit: bool,
}

pub enum Command {
    None,
    Future(BoxFuture<BoxMessage>),
    Message(BoxMessage),
}

impl Command {
    pub fn boxed<F>(f: F) -> Self
    where
        F: Future<Output = BoxMessage> + Send + 'static,
    {
        Self::Future(Box::pin(f))
    }
}

impl App {
    pub fn init() -> Self {
        // init logging
        // init terminal
        // get cmd args
        // init config

        let (msg_tx, msg_rx) = tokio::sync::mpsc::unbounded_channel();

        let messages = UnboundedReceiverStream::new(msg_rx);

        Self {
            messages,

            layout: ComponentTree::new(),
            components: Components::new(),

            suspended: false,
            should_quit: false,
        }
    }

    pub fn terminate(&mut self) {
        // terminate terminal
        // terminate logging
    }

    pub async fn run(&mut self) -> Result<i32> {
        // init terminal

        Ok(0)
    }

    pub async fn view(&self) -> Result<()> {
        Ok(())
    }

    pub async fn update(&mut self, action: BoxMessage) -> Command {
        if let Some(action) = action.as_any().downcast_ref::<AppMessage>() {
            match action {
                AppMessage::Quit => {
                    self.should_quit = true;
                }
                AppMessage::Suspend => {
                    self.suspended = true;
                }
                AppMessage::Resume => {
                    self.suspended = false;
                }
                _ => (),
            }
        } else {
            todo!()
        }

        println!("{:?}", action);

        Command::None
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.terminate();
    }
}

pub fn process_command(cmd: Command, tx: UnboundedSender<BoxMessage>) {
    match cmd {
        Command::Future(fut) => {
            tokio::spawn(async move {
                let msg = fut.await;
                tx.send(msg).expect("failed to send message");
            });
        }

        Command::Message(msg) => {
            tokio::spawn(async move {
                tx.send(msg).expect("failed to send message");
            });
        }

        Command::None => (),
    }
}
