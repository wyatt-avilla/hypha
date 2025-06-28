use actix_web::{App, HttpServer, Responder, get};
use api::ServiceStatuses;
use clap::Parser;
use itertools::Itertools;

mod systemd;

/// Simple systemd service monitor
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Space delimited service names to monitor (e.g syncthing.service)
    #[arg(short, long, required = true, value_parser, num_args = 1.., value_delimiter = ' ')]
    pub services: Vec<String>,

    /// Port to run the server on
    #[arg(short, long, default_value_t = 8910)]
    port: u16,

    /// Number of workers for the server
    #[arg(short, long, default_value_t = 1)]
    workers: usize,

    /// Log level, one of (INFO, WARN, ERROR, DEBUG, TRACE)
    #[arg(short, long, default_value_t = tracing::Level::INFO)]
    log_level: tracing::Level,
}

#[get("/")]
async fn root_endpoint() -> impl Responder {
    ServiceStatuses {
        service_to_alive: [("s1".to_string(), true), ("s2".to_string(), false)].into(),
    }
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    tracing_subscriber::fmt()
        .with_max_level(args.log_level)
        .init();

    let systemd_interface = systemd::ServiceMonitorInterface::new(&args.services).await?;
    tracing::info!(
        "Monitoring services: [{}]",
        systemd_interface.monitored_services().join(", ")
    );

    Ok(HttpServer::new(|| App::new().service(root_endpoint))
        .server_hostname("hypha_server")
        .bind(("127.0.0.1", args.port))?
        .workers(args.workers)
        .run()
        .await?)
}
