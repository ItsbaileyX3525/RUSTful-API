use actix_web::{get, post, delete, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use actix_files;
use rand::{seq::IndexedMutRandom};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;
use rustls::{ServerConfig, pki_types::{PrivateKeyDer, CertificateDer}};
use rustls_pemfile::{certs, private_key};
use std::fs::File;
use std::io::BufReader;

#[get("/greet/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", name.as_str()))
}

#[derive(Serialize, Deserialize, Clone)]
struct Quote {
    id: String,
    text: String,
    speaker: String,
}

#[derive(Deserialize)]
struct NewQuote {
    text: String,
    speaker: String,
}

use std::collections::HashMap;

struct AppState {
    quotes: Mutex<Vec<Quote>>,
    url_map: Mutex<HashMap<String, String>>, // short -> original
}

#[get("/quotes")]
async fn get_quotes(data: web::Data<AppState>) -> impl Responder {
    let quotes = data.quotes.lock().unwrap();
    HttpResponse::Ok().json(quotes.clone())
}

#[get("/quote")]
async fn get_random_quote(data: web::Data<AppState>) -> impl Responder {
    let mut quotes = data.quotes.lock().unwrap();
    
    if let Some(quote) = quotes.choose_mut(&mut rand::rng()) {
        HttpResponse::Ok().json(quote)
    } else {
        HttpResponse::NotFound().body("No quotes found.")
    }
}

#[get("/quotes/{id}")]
async fn get_quote_by_id(data: web::Data<AppState>, path: web::Path<String>,) -> impl Responder {
    let id = path.into_inner();
    let quotes = data.quotes.lock().unwrap();

    if let Some(quote) = quotes.iter().find(|q| q.id == id) {
        HttpResponse::Ok().json(quote.clone())
    } else {
        HttpResponse::NotFound().body(format!("Quote with ID {} not found", id))
    }
}

#[post("/quotes")]
async fn add_quote(data: web::Data<AppState>, new_quote: web::Json<NewQuote>,) -> impl Responder {
    let mut quotes = data.quotes.lock().unwrap();
    
    let quote = Quote {
        id: Uuid::new_v4().to_string(),
        text: new_quote.text.clone(),
        speaker: new_quote.speaker.clone(),
    };
    
    quotes.push(quote.clone());
    
    HttpResponse::Created().json(quote)
}


#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
struct ShortenResponse {
    short: String,
    url: String,
}

#[post("/shorten")]
async fn shorten_url(data: web::Data<AppState>, req: web::Json<ShortenRequest>) -> impl Responder {
    let mut url_map = data.url_map.lock().unwrap();
    let short = Uuid::new_v4().to_string()[..5].to_string();
    url_map.insert(short.clone(), req.url.clone());
    HttpResponse::Ok().json(ShortenResponse { short, url: req.url.clone() })
}

#[get("/path/{short}")]
async fn get_shortened_url(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let short = path.into_inner();
    let url_map = data.url_map.lock().unwrap();
    if let Some(url) = url_map.get(&short) {
        HttpResponse::Found().append_header(("Location", url.clone())).finish()
    } else {
        HttpResponse::NotFound().body("Short URL not found.")
    }
}

#[delete("/shorten/{short}")]
async fn delete_short_url(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let short = path.into_inner();
    let mut url_map = data.url_map.lock().unwrap();
    if url_map.remove(&short).is_some() {
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().body("Short URL not found.")
    }
}


use std::fs::metadata;
fn load_rustls_config() -> Option<rustls::ServerConfig> {
    let cert_path = "ssl/cert.pem";
    let key_path = "ssl/key.pem";
    if metadata(cert_path).is_err() || metadata(key_path).is_err() {
        return None;
    }
    let cert_file = &mut BufReader::new(File::open(cert_path).expect("Failed to open cert.pem"));
    let key_file = &mut BufReader::new(File::open(key_path).expect("Failed to open key.pem"));

    let cert_chain: Vec<CertificateDer> = certs(cert_file)
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse certificate");

    let private_key: PrivateKeyDer = private_key(key_file)
        .expect("Failed to parse private key")
        .expect("No private key found");

    Some(ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, private_key)
        .expect("Failed to create TLS configuration"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let initial_quotes = vec![
        Quote {
            id: Uuid::new_v4().to_string(),
            text: "Bazinga, punk!".to_string(),
            speaker: "Sheldon Cooper".to_string(),
        },
        Quote {
            id: Uuid::new_v4().to_string(),
            text: "You may be from Texas but I'm from New Jearsey!".to_string(),
            speaker: "Leonard Hofstadter".to_string(),
        },
    ];

    let app_state = web::Data::new(AppState {
        quotes: Mutex::new(initial_quotes),
        url_map: Mutex::new(HashMap::new()),
    });

    match load_rustls_config() {
        Some(config) => {
            println!("Starting HTTPS server at https://0.0.0.0:443");
            println!("Starting HTTP server at http://0.0.0.0:80");

            let https_server = HttpServer::new({
                let app_state = app_state.clone();
                move || {
                    App::new()
                        .wrap(
                            Cors::default()
                                .allow_any_origin()
                                .allow_any_method()
                                .allow_any_header()
                                .supports_credentials()
                        )
                        .app_data(app_state.clone())
                        .service(get_quotes)
                        .service(get_random_quote)
                        .service(get_quote_by_id)
                        .service(add_quote)
                        .service(greet)
                        .service(shorten_url)
                        .service(get_shortened_url)
                        .service(delete_short_url)
                        // Static files must go last for some reason
                        .service(actix_files::Files::new("/src/styles", "./src/styles"))
                        .service(actix_files::Files::new("/", "./src").index_file("index.html"))
                }
            })
            .bind_rustls_0_23(("0.0.0.0", 443), config)?
            .run();

            let http_server = HttpServer::new(move || {
                App::new()
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                            .supports_credentials()
                    )
                    .app_data(app_state.clone())
                    .service(get_quotes)
                    .service(get_random_quote)
                    .service(get_quote_by_id)
                    .service(add_quote)
                    .service(greet)
                    .service(shorten_url)
                    .service(get_shortened_url)
                    .service(delete_short_url)
                    // Static files must go last for some reason
                    .service(actix_files::Files::new("/", "./src").index_file("index.html"))
            })
            .bind(("0.0.0.0", 80))?
            .run();

            // Run both servers concurrently
            let (_https, _http) = tokio::try_join!(https_server, http_server)?;
            Ok(())
        }
        None => {
            println!("SSL not found, starting HTTP server at http://0.0.0.0:840");
            HttpServer::new(move || {
                App::new()
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header()
                            .supports_credentials()
                    )
                    .app_data(app_state.clone())
                    .service(get_quotes)
                    .service(get_random_quote)
                    .service(get_quote_by_id)
                    .service(add_quote)
                    .service(greet)
                    .service(shorten_url)
                    .service(get_shortened_url)
                    .service(delete_short_url)
                    // Static files must go last for some reason
                    .service(actix_files::Files::new("/", "./src").index_file("index.html"))
            })
            .bind(("0.0.0.0", 840))?
            .run()
            .await
        }
    }
}
