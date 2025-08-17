use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files;
use rand::{seq::IndexedMutRandom};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

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

struct AppState {
    quotes: Mutex<Vec<Quote>>,
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
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_quotes)
            .service(get_random_quote)
            .service(get_quote_by_id)
            .service(add_quote)
            .service(greet)

            // Static files must go last for some reason
            .service(actix_files::Files::new("/src/styles", "./src/styles"))
            .service(actix_files::Files::new("/", "./src").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8989))?
    .run()
    .await
}
