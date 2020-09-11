use actix_web::{web, App, HttpRequest, HttpServer, Responder};

/// Handles all routes
async fn forward(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let method = req.method().as_str().to_owned();
    let path = req.path().to_owned();
    let headers = req.headers().clone();

    for (key, value) in headers.iter() {
    	println!("{}: {}", key, value.to_str().unwrap());
    }

    method + " " + &path
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .default_service(web::route().to(forward))
    })
    .bind(("0.0.0.0", 8088))?
    .system_exit()
    .run()
    .await
}