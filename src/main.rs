use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use env_logger::{Builder, Env};
use log::info;

const BINDING_ADDRESS: &str = "localhost:8080";

async fn hey(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hey there!")
}

async fn hey_name(req: HttpRequest) -> HttpResponse {
    let name = req.match_info().query("name");
    HttpResponse::Ok().body(format!("Hey {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    // Using SPIDER_LOG_LEVEL envar to determine the log level, default
    // to all logs passing through (trace is the highest level)
    let env = Env::default().filter_or("SPIDER_LOG_LEVEL", "orb=warn");
    Builder::from_env(env).init();

    info!("Starting spider server: {}", BINDING_ADDRESS);
    HttpServer::new(|| {
        App::new()
            .route("/hey", web::get().to(hey))
            .route("/hey/{name}", web::get().to(hey_name))
    })
    .bind(BINDING_ADDRESS)?
    .run()
    .await
}

#[cfg(test)]
mod handler_tests {
    use super::*;
    use actix_web::{body::Body, http, test};

    #[actix_rt::test]
    async fn test_hey() {
        let req = test::TestRequest::get().to_http_request();
        let mut resp = hey(req).await;
        let response_body = resp.take_body();
        let body = response_body.as_ref().unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(&Body::from("Hey there!"), body);
    }

    #[actix_rt::test]
    async fn test_hey_name() {
        let req = test::TestRequest::default()
            .param("name", "Dave")
            .to_http_request();
        let mut resp = hey_name(req).await;
        let response_body = resp.take_body();
        let body = response_body.as_ref().unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(&Body::from("Hey Dave!"), body);
    }
}
