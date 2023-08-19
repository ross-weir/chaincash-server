//! ChainCash server CLI.
use clap::Parser;
use tracing::info;

/// ChainCash payment server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The address to listen on
    #[clap(short, long, default_value = "127.0.0.1:8080")]
    listen: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    info!("started with {:?}", args);

    // listenfd is used to enable auto-reloading in development
    // otherwise fallback to standard tcp listener
    let listener = listenfd::ListenFd::from_env()
        .take_tcp_listener(0)
        .unwrap()
        .unwrap_or_else(|| std::net::TcpListener::bind(args.listen).unwrap());

    info!("listening on {:?}", listener.local_addr().unwrap());

    chaincash_server::serve_blocking(listener).await.unwrap();
}
