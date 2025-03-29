#[macro_use]
extern crate log;

use clap::Parser;
use std::env;

mod args;
#[cfg(feature = "bundle")]
mod embedded;
mod server;

fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    if env::var("RUST_LOG").is_err() {
        builder.filter_level(log::LevelFilter::Info);
    }
    builder.init();

    let args = args::Args::parse();

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .max_blocking_threads(args.blocking_threads)
        .build()
        .expect("Failed to create Tokio runtime");

    if let Err(e) = runtime.block_on(async_main(args)) {
        error!("Server error: {}", e);
        std::process::exit(1);
    }
}

async fn async_main(args: args::Args) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting with args: {:?}", args);

    server::start_server(&args).await
}
