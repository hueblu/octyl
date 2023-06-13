use anyhow::Result;

use octyl::App;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging()?;

    let mut app = App::new();

    app.run().await?;
    Ok(())
}

fn setup_logging() -> Result<()> {
    // check if the file exists by calling metadata on it

    let result = std::fs::OpenOptions::new()
        .write(true)
        .append(false)
        .open("log.txt");

    let file = match result {
        Ok(file) => file,
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => std::fs::File::create("log.txt")?,
            _ => {
                return Err(e.into());
            }
        },
    };

    let collector = tracing_subscriber::fmt()
        .with_writer(file)
        .with_max_level(tracing::Level::TRACE)
        .with_ansi(true)
        .finish();

    tracing::subscriber::set_global_default(collector).expect("setting default subscriber failed");

    std::panic::set_hook(Box::new(|panic| {
        if let Some(location) = panic.location() {
            tracing::error!(
                message = "panic occurred",
                reason = %panic,
                file = %location.file(),
                line = %location.line(),
                column = %location.column(),
            );
        } else {
            tracing::error!(message = "panic occurred", reason = %panic);
        }
    }));

    Ok(())
}
