use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};
use clap::Parser;
use itertools::Itertools;
use tokio::sync::Mutex as TokioMutex;

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

#[get("/api")]
async fn root_endpoint(
    req: HttpRequest,
    data: web::Data<TokioMutex<systemd::ServiceMonitorInterface<'_>>>,
) -> impl Responder {
    tracing::info!(
        "Request from IP: {}",
        req.connection_info()
            .realip_remote_addr()
            .unwrap_or("Unknown ip")
    );

    match data.lock().await.get_service_statuses().await {
        Ok(s) => HttpResponse::Ok().json(s),
        Err(e) => {
            tracing::error!("{e}");
            HttpResponse::InternalServerError().body(format!("Error: {e}"))
        }
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

    let app_data = web::Data::new(TokioMutex::new(systemd_interface));

    Ok(
        HttpServer::new(move || App::new().app_data(app_data.clone()).service(root_endpoint))
            .server_hostname("hypha_server")
            .bind(("127.0.0.1", args.port))?
            .workers(args.workers)
            .run()
            .await?,
    )
}
