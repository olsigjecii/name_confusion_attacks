use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

// Import our simulated external crates
mod malicious_logger;
mod secure_logger;

async fn vulnerable_handler() -> impl Responder {
    // SCENARIO: The developer made a typo in Cargo.toml or import
    // and is using the malicious logger.
    
    // This looks innocent, but triggers the payload.
    malicious_logger::logger::info("User accessed vulnerable endpoint");
    
    HttpResponse::Ok().body("Vulnerable Endpoint Executed. Check your server logs!")
}

async fn secure_handler() -> impl Responder {
    // SCENARIO: The developer verified the crate and is using the legitimate one.
    
    secure_logger::logger::info("User accessed secure endpoint");
    
    HttpResponse::Ok().body("Secure Endpoint Executed. Everything is safe.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Let's set a fake secret to demonstrate the theft
    unsafe {
        env::set_var("API_SECRET_KEY", "12345-SUPER-SECRET-PASSWORD");
        env::set_var("AWS_ACCESS_TOKEN", "AKIA-FAKE-TOKEN-EXAMPLE");
    }

    println!("Server running at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .route("/vulnerable", web::get().to(vulnerable_handler))
            .route("/secure", web::get().to(secure_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}