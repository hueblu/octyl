#![allow(dead_code)]

use anyhow::Result;
use octyl::app::App;

const LOG_LEVEL: u64 = 1;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logging(LOG_LEVEL)?;

    let code = App::new()?.run().await?;
    std::process::exit(code);
}

fn setup_logging(verbosity: u64) -> Result<()> {
    let base_config = fern::Dispatch::new().level(match verbosity {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    });

    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {}]({}) {}",
                record.level(),
                record.target(),
                chrono::offset::Local::now().format("%b-%d %H:%M:%S"),
                message,
            ))
        })
        .chain(fern::log_file("log")?);

    base_config.chain(file_config).apply()?;

    Ok(())
}
