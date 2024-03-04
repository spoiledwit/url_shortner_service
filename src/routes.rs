use crate::{utils, db};
use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use tokio_postgres::Client;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use actix_web::http::header::LOCATION;

#[derive(Deserialize)]
pub struct UrlData {
    long_url: String,
}

#[derive(Serialize)]
pub struct ShortUrlResponse {
    short_url: String,
}


pub async fn shorten_url(client: web::Data<Arc<Client>>, data: web::Json<UrlData>) -> impl Responder {
    match db::check_long_url_existence(&client, &data.long_url).await {
        Ok(Some(existing_short_url)) => {
            HttpResponse::Ok().json(ShortUrlResponse { short_url: existing_short_url })
        },
        Ok(None) => {
            let salt = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string();
            let short_url = utils::generate_short_url(&data.long_url, &salt);

            match db::insert_url(&client, &data.long_url, &short_url).await {
                Ok(_) => HttpResponse::Ok().json(ShortUrlResponse { short_url }),
                Err(e) => {
                    eprintln!("Failed to insert URL into database: {}", e);
                    HttpResponse::InternalServerError().finish()
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to check long URL existence in database: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn redirect_to_original(client: web::Data<Arc<Client>>, short_url_path: web::Path<String>) -> impl Responder {
    let short_url = short_url_path.into_inner();

    match db::find_original_url(&client, &short_url).await {
        Ok(Some(original_url)) => {
            HttpResponse::Found().append_header((LOCATION, original_url)).finish()
        },
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(e) => {
            eprintln!("Failed to retrieve original URL from database: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}