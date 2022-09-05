use actix_web::{get, Responder};

#[get("/index.html")]
pub(crate) async fn index() -> impl Responder {
    format!("Hello world")
}
