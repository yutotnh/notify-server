use clap::Parser;
use notify_rust::Notification;

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct NotificationInfo {
    summary: Option<String>,
    body: Option<String>,
}

async fn index(info: web::Query<NotificationInfo>) -> HttpResponse {
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

    notifycation.show().unwrap();

    HttpResponse::Ok().json(NotificationInfo {
        summary: Some(summary.to_string()),
        body: Some(body.to_string()),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // 通知を作成して送信
    Notification::new()
        .summary("Notify server has been activated.")
        .show()
        .unwrap();

    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind(("0.0.0.0", args.port))?
        .run()
        .await
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
