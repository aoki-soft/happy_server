use actix_files;
use actix_web::{App, HttpServer, dev::Server};

// Compile-time defaults
#[cfg(any(feature = "japanese",not(feature = "english")))]
const COMPILE_TIME_DEFAULT_LANGUAGE:Language = Language::Japanese;
#[cfg(feature = "english")]
const COMPILE_TIME_DEFAULT_LANGUAGE:Language = Language::English;

/// Language settings
#[allow(dead_code)]
enum Language {
    Japanese,
    English
}

/// # application entry point
/// Note: Although it is an async function, it is converted to a normal function signature by the #[actix_web::main] attribute.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // language setting
    let language = COMPILE_TIME_DEFAULT_LANGUAGE;

    // start web server
    let web_server = start_server().await;
    match web_server {
        Err(_) => {
            // Output when the web server fails to start.
            // case of cli
            let output_message = match language{
                Language::Japanese => format!("Error: カレントディレクトリをhttpで配信できませんでした。"),
                Language::English => format!("Error: The current directory could not be delivered via http.")
            };
            println!("{}", output_message);
            std::process::exit(0);
        },
        Ok(web_server) => {
            // Output when the web server is successfully started.
            // case of cli
            let output_message = match language{
                Language::Japanese => format!("Running: カレントディレクトリをhttpで配信しています。\n\
                http://localhost にアクセスすればブラウズができます。\n\n\
                終了する場合は、Ctrl + C を押すか、このウィンドを閉じてください。"),
                Language::English => format!("Running: The current directory is served by http!!\n\
                You can browse by visiting http://localhost. \n\n\
                To exit, press Ctrl + C or close this window.")
            };
            println!("{}", output_message);

            // wait
            web_server.await
        }
    }
}

/// # start web server
/// # Returns
/// * webserver handler or err
async fn start_server() -> std::io::Result<Server> {
    Ok(HttpServer::new(|| {
        App::new().service(actix_files::Files::new("/", ".").show_files_listing())
    })
    .bind("0.0.0.0:80")?
    .run())
}