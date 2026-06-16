use tokio::signal;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    config: Option<String>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    let config = fflags::utils::Config::new(args.config.as_deref());
    let adapters = fflags::adapters::Adapters::new(&config);
    let services = fflags::app::Services::new(&config, adapters);

    let bind_ip = &config.api.bind_ip;
    let port = &config.api.port;

    let app = fflags::api::init_router(fflags::api::AppState{
        config: config.clone(),
        services: services,
    });

    let listener = tokio::net::TcpListener::bind(format!("{bind_ip}:{port}")).await.expect(&format!("Could not start server on IP {bind_ip} and port {port}"));
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn shutdown_signal() {
    let sigint = async {
        signal::ctrl_c()
            .await
            .expect("Failed to register SIGINT handler");
    };

    #[cfg(unix)]
    let sigterm = async {
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
