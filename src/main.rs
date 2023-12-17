use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::Arc;
mod db;
mod utils;
mod routes;


async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust URL Shortener!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = match db::establish_connection().await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection failed"));
        }
    };
    let shared_client = Arc::new(client);
    
    println!("Database connection established.");

    match db::create_urls_table(&shared_client).await {
        Ok(_) => println!("Created the urls table."),
        Err(err) => {
            eprintln!("Couldn't create the urls table: {}", err);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Couldn't create the urls table"));
        }
    };
    
    match HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_client.clone()))
            .route("/", web::get().to(greet))
            .route("/shorten", web::post().to(routes::shorten_url))
            .route("/{short_url}", web::get().to(routes::redirect_to_original))
    })

    .bind("0.0.0.0:8080") {
        Ok(server) => {
            println!("Server running at http://127.0.0.1:8080/");
            if let Err(e) = server.run().await {
                eprintln!("Error running the server: {}", e);
            }
        },
        Err(err) => eprintln!("Couldn't start the server: {}", err),
    }

    Ok(())
}