// use axum::{Router, routing::get};
use tokio::signal;
use clap::Parser;
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    log_level: Option<String>
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = ff::utils::Config::new(None);

    let bind_ip = "0.0.0.0";
    let port = "3000";

    let app = ff::api::init_router();
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", bind_ip, port)).await.expect(&format!("Could not start server on IP {} and port {}", bind_ip, port));
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let sigint = async {
        signal::ctrl_c()
            .await
            .expect("Failed to register SIGINT handler");
    };

    #[cfg(unix)]
    let mut sigterm = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to register SIGTERM handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let sigterm = std::future::pending::<()>();

    tokio::select! {
        _ = sigint => {
            println!("Received SIGINT");
        },
        _ = sigterm => {
            println!("Received SIGTERM");
        },
    }
}
