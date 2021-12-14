use actix_files::Files;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use env_logger::{Builder, Env};
use log::info;
use tera::{Context, Tera};

const BINDING_ADDRESS: &str = "localhost:8080";

struct AppState {
    app_name: String,
}

async fn hey() -> HttpResponse {
    HttpResponse::Ok().body("Hey there!")
}

async fn hey_name(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let app_name = &data.app_name;
    let name = req.match_info().query("name");
    HttpResponse::Ok().body(format!("Hey {}! From {}", name, app_name))
}

async fn homepage() -> HttpResponse {
    let homepage_html = include_str!("../views/index.html");
    HttpResponse::Ok()
        .content_type("text/html; charset=UTF-8")
        .body(homepage_html)
}

async fn html_bye(req: HttpRequest) -> HttpResponse {
    let name = req.match_info().query("name");
    let bye_template = include_str!("../views/templates/bye.html");
    let mut tera_context = Context::new();
    tera_context.insert("name", name);
    match Tera::one_off(bye_template, &tera_context, true) {
        Err(..) => HttpResponse::InternalServerError().finish(),
        Ok(interpolated_html) => HttpResponse::Ok()
            .content_type("text/html; charset=UTF-8")
            .body(interpolated_html),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    // Using SPIDER_LOG_LEVEL envar to determine the log level, default
    // to all logs passing through (trace is the highest level)
    let env = Env::default().filter_or("SPIDER_LOG_LEVEL", "orb=trace");
    Builder::from_env(env).init();

    info!("Starting orb server: http://{}", BINDING_ADDRESS);
    HttpServer::new(|| {
        App::new()
            // Share AppState among all routes
            .data(AppState {
                app_name: String::from("orb"),
            })
            .route("/", web::get().to(homepage))
            .route("/hey", web::get().to(hey))
            // Optionally restrict methods to a certain route
            // .route("/hey", web::post().to(|| HttpResponse::MethodNotAllowed())). By default they 404
            .route("/hey/{name}", web::get().to(hey_name))
            .route("/bye/{name}", web::get().to(html_bye))
            // The serve_from arg (second arg) is the directory on system from which files will be served.
            // it's relative to the project root (wherever Cargo.toml is). The first arg is the server root path
            // from which files will be served
            .service(Files::new("/static", "./static"))
        // Optionally set a default response, should nothing match the above definitions
        // The default is 404 if not specified, but this can be changed
        //.default_service(web::to(|| HttpResponse::MethodNotAllowed()))
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
        let mut resp = hey().await;
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
        let mut resp = hey_name(
            req,
            web::Data::new(AppState {
                app_name: String::from("orb test"),
            }),
        )
        .await;
        let response_body = resp.take_body();
        let body = response_body.as_ref().unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(&Body::from("Hey Dave! From orb test"), body);
    }

    #[actix_rt::test]
    async fn test_homepage() {
        let mut resp = homepage().await;
        let response_body = resp.take_body();
        let body = response_body.as_ref().unwrap();
        let expected_body_str = include_str!("../views/index.html");
        assert_eq!(resp.status(), http::StatusCode::OK);
        assert_eq!(&Body::from(expected_body_str), body);
    }
}
