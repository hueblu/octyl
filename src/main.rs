use anyhow::Result;
use clap::Parser;
use octyl::{
    app::{App, Args},
    utils::{initialize_logging, initialize_panic_handler},
};

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logging()?;

    initialize_panic_handler();

    let args = Args::parse();
    let mut app = App::new(args)?;
    app.run().await?;

    Ok(())
}
