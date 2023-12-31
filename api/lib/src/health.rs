use actix_web::{get, HttpResponse,web::ServiceConfig};

pub const API_VERSION: &str = "v0.0.1";

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e),
    }
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "v0.0.1"))
        .finish()
}

pub fn service(cfg: &mut ServiceConfig){
    cfg.service(health);
}



