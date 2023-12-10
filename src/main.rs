use clap::Parser;
use notify_rust::Notification;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // 通知を作成して送信
    Notification::new()
        .summary("Notify server has been activated.")
        .show()
        .unwrap();

    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", args.port))
        .unwrap_or_else(|_| panic!("Failed to bind port {}.", args.port))
        .run()
        .await
}

#[derive(Debug, Deserialize, Serialize)]
struct NotificationInfo {
    summary: Option<String>,
    body: Option<String>,
}

#[get("/")]
/// 通知を作成して送信
///
/// # Arguments
/// * `info` - 通知の情報
/// * `req` - リクエスト
/// # Returns
/// * `HttpResponse` - 通知の情報
async fn index(info: web::Query<NotificationInfo>, req: actix_web::HttpRequest) -> HttpResponse {
    let mut notifycation = Notification::new();

    let summary = match info.summary {
        Some(ref s) => {
            notifycation.summary(s);
            s
        }
        None => "",
    };

    let body = match info.body {
        Some(ref s) => {
            notifycation.body(s);
            s
        }
        None => "",
    };

    if req.connection_info().peer_addr().is_some() {
        if body.is_empty() {
            notifycation.body(&format!(
                "Received request from {addr}",
                addr = req.connection_info().peer_addr().unwrap()
            ));
        } else {
            notifycation.body(&format!(
                "{body}\n\nReceived request from {addr}",
                addr = req.connection_info().peer_addr().unwrap()
            ));
        }
    }

    notifycation.show().unwrap();

    HttpResponse::Ok().json(NotificationInfo {
        summary: Some(summary.to_string()),
        body: Some(body.to_string()),
    })
}

#[derive(Debug, Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = false,
)]
struct Args {
    /// The port to listen on
    #[clap(default_value_t = 12413)]
    port: u16,
}
