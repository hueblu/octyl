// TODO: config lazy statics

use anyhow::Result;

use octyl::{app::App, util};

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: initialize logging
    // TODO: initialize panic handler
    // TODO: cli args

    util::init_logging()?;

    let mut app = App::new()?;
    app.run().await?;

    Ok(())
}
