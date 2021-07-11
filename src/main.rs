use actix_files;
use actix_web::{App, HttpServer};

/// # application entry point
/// Note: Although it is an async function, it is converted to a normal function signature by the #[actix_web::main] attribute.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Runnig: The current directory is served by http!!\n\
    You can browse by visiting http://localhost. \n\n\
    To exit, press Ctrl + C or close this window.");

    HttpServer::new(|| {
        App::new().service(actix_files::Files::new("/", ".").show_files_listing())
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
} 