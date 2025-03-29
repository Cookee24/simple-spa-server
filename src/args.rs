use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Args {
    /// The listen address for the server.
    #[arg(long, short, default_value = ":8080")]
    pub listen: String,

    /// The serve directory for the server.
    #[arg(long, short, default_value = ".")]
    pub serve_dir: String,

    /// If return index.html if not matched.
    #[arg(long, default_value_t = true)]
    pub index: bool,

    /// The maximum number of blocking threads for the Tokio runtime.
    #[arg(long, default_value_t = 8)]
    pub blocking_threads: usize,
}
