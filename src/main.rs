use actix_web::web::Path;
use actix_web::Result;
use actix_web::{error, get, App, HttpServer};
use failure::Fail;
use std_logger;
use tracing::instrument;

#[derive(Fail, Debug)]
#[fail(display = "Hello user. You got an error.")]
pub struct MyError {
    name: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for MyError {}

#[instrument]
#[get("/{id}")]
async fn index(path: Path<String>) -> Result<&'static str, MyError> {
    let id = path.to_string();
    if id == "good_path".to_owned() {
        return Ok("nice");
    }
    let err = MyError { name: "test error" };
    Err(err)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
