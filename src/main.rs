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
        Some(ref s) => s,
        None => "",
    };

    let body = add_ip_addr_info(req.peer_addr(), body);

    notifycation.body(body.as_str());

    notifycation.show().unwrap();

    HttpResponse::Ok().json(NotificationInfo {
        summary: Some(summary.to_string()),
        body: Some(body.to_string()),
    })
}

/// IPアドレスの情報を追加する
/// # Arguments
/// * `socket` - ソケットの情報
/// * `body` - 通知の本文
/// # Returns
/// * `String` - bodyにIPアドレスを付加した文字列
/// # Note
/// * `socket`が`None`の場合は`body`をそのまま返す
/// * `socket`が`Some`の場合は`body`の最終行から1行を空けてに`Received request from {addr}`を追加して返す
fn add_ip_addr_info(socket: Option<std::net::SocketAddr>, body: &str) -> String {
    match socket {
        Some(addr) => {
            if body.is_empty() {
                format!("Received request from {ip}", ip = addr.ip())
            } else {
                format!(
                    "{body}\n\nReceived request from {ip}",
                    ip = addr.ip(),
                    body = body
                )
            }
        }
        None => body.to_string(),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::{test, App};

    #[test]
    async fn test_add_ip_addr_info() {
        // 引数がNoneの場合
        assert_eq!(add_ip_addr_info(None, ""), "");

        assert_eq!(add_ip_addr_info(None, "test"), "test");

        // 引数がSomeの場合
        let socket = std::net::SocketAddr::from(([192, 168, 0, 1], 12345));
        assert_eq!(
            add_ip_addr_info(Some(socket), ""),
            "Received request from 192.168.0.1"
        );

        assert_eq!(
            add_ip_addr_info(Some(socket), "test"),
            "test\n\nReceived request from 192.168.0.1"
        );

        assert_eq!(
            add_ip_addr_info(Some(socket), "test\ntest"),
            "test\ntest\n\nReceived request from 192.168.0.1"
        );

        let socket = std::net::SocketAddr::new(
            std::net::IpAddr::V6(std::net::Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
            8080,
        );

        assert_eq!(
            add_ip_addr_info(Some(socket), ""),
            "Received request from ::1"
        );

        assert_eq!(
            add_ip_addr_info(Some(socket), "test"),
            "test\n\nReceived request from ::1"
        );
    }

    #[actix_web::test]
    async fn test_index_empty() {
        let app = actix_web::test::init_service(App::new().service(index)).await;

        let req = actix_web::test::TestRequest::get().uri("/").to_request();
        let resp = actix_web::test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = actix_web::test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(body, "{\"summary\":\"\",\"body\":\"\"}");
    }

    #[actix_web::test]
    async fn test_index_summary_and_body() {
        let app = actix_web::test::init_service(App::new().service(index)).await;

        let req = actix_web::test::TestRequest::get()
            .uri("/?summary=summary-test&body=body-test")
            .to_request();
        let resp = actix_web::test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = actix_web::test::read_body(resp).await;
        let body = String::from_utf8(body.to_vec()).unwrap();

        assert_eq!(
            body,
            "{\"summary\":\"summary-test\",\"body\":\"body-test\"}"
        );
    }
}
