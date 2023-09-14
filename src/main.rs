use anyhow::Result;
// use clap::Parser;
use octyl::{
    app::App,
    utils::{
        initialize_logging,
        initialize_panic_handler,
        // version,
    },
};

/// Define the command line arguments structure
// #[derive(Parser, Debug)]
// #[command(version = version(), about = "text editor")]
// struct Args {}

#[tokio::main]
async fn main() -> Result<()> {
    initialize_logging()?;

    initialize_panic_handler();

    // let args = Args::parse();
    let mut app = App::new()?;
    app.run().await?;

    Ok(())
}
// ANCHOR_END: all
