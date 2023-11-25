use anyhow::{Context, Result};

use tracing::subscriber::set_global_default;

use crate::components::logger::GlobalLogWriter;

pub fn init_logging() -> Result<()> {
    let subscriber = tracing_subscriber::fmt().with_writer(|| GlobalLogWriter);
    set_global_default(subscriber.finish()).context("subscrober failled")
}
