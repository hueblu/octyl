use anyhow::Result;
use octyl::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::init();
    std::process::exit(app.run().await?);
}
