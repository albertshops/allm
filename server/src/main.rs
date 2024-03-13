use actix::{Actor, StreamHandler};
use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::env;
use uuid::Uuid;

struct ChatWebSocket;
impl Actor for ChatWebSocket {
    type Context = ws::WebsocketContext<Self>;
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWebSocket {
    fn handle(&mut self, message: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = message {
            match serde_json::from_str::<Message>(&text.to_string()) {
                Ok(message) => ctx.text("VALID"),
                _ => ctx.text("INVALID"),
            }
        }
    }
}

#[derive(Serialize)]
struct Chat {
    id: Uuid,
    model: String,
}

#[derive(Deserialize, Debug)]
struct Message {
    id: Uuid,
    content: String,
}

#[post("/chat")]
async fn create_chat(pool: web::Data<PgPool>) -> HttpResponse {
    let chat = sqlx::query_as!(
        Chat,
        "INSERT INTO chats (id, model) VALUES ($1, 'llama2') RETURNING *;",
        Uuid::new_v4(),
    )
    .fetch_one(pool.get_ref())
    .await;

    match chat {
        Ok(chat) => HttpResponse::Ok().json(chat),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/ws")]
async fn ws_chat(request: HttpRequest, stream: web::Payload) -> impl Responder {
    ws::start(ChatWebSocket, &request, stream)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&connection_string)
        .await
        .expect("failed to connect to postgres");

    let pool = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .service(create_chat)
            .service(ws_chat)
            .app_data(pool.clone())
    })
    .bind(("127.0.0.1", 6600))?
    .run()
    .await
}
