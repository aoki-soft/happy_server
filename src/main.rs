// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use actix_files;
use actix_web::{App, HttpServer, dev::Server};
// Add color to console output
use colored::*;

// Compile-time defaults
#[cfg(any(feature = "japanese",not(feature = "english")))]
const COMPILE_TIME_DEFAULT_LANGUAGE:Language = Language::Japanese;
#[cfg(feature = "english")]
const COMPILE_TIME_DEFAULT_LANGUAGE:Language = Language::English;

#[cfg(not(feature = "no_color"))]
const COMPILE_TIME_DEFAULT_COLOR: bool = true;
#[cfg(feature = "no_color")]
const COMPILE_TIME_DEFAULT_COLOR: bool = false;



/// Language settings
#[allow(dead_code)]
enum Language {
    Japanese,
    English
}

#[allow(dead_code)]
struct StyledString {
    error: String,
    note: String,
    running: String,
    finish: String
} impl StyledString {
    fn colored() -> Self {
        Self{
            error: "Error".red().bold().to_string(),
            note: "Note".blue().bold().to_string(),
            running: "Running".green().bold().to_string(),
            finish: "Finish".green().bold().to_string(),
        }
    }
    fn no_colored() -> Self {
        Self {
            error: "Error".to_string(),
            note: "Note".to_string(),
            running: "Running".to_string(),
            finish: "Finish".to_string(),
        }
    }
}

/// # application entry point
/// Note: Although it is an async function, it is converted to a normal function signature by the #[actix_web::main] attribute.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // language setting
    let language = COMPILE_TIME_DEFAULT_LANGUAGE;
    // cli color setting  ex) true = colored , false = no_colored
    let cli_color = COMPILE_TIME_DEFAULT_COLOR;

    // cli style strings
    let styled_string = if cli_color {
        StyledString::colored()
    } else {
        StyledString::no_colored()
    };

    // start web server
    let web_server = start_server().await;
    match web_server {
        Err(_) => {
            // Output when the web server fails to start.
            // case of cli
            let output_message = match language{
                Language::Japanese => format!("{error}: カレントディレクトリをhttpで配信できませんでした。", error=styled_string.error),
                Language::English => format!("{error}: The current directory could not be delivered via http.", error=styled_string.error)
            };
            println!("{}", output_message);
            std::process::exit(0);
        },
        Ok(web_server) => {
            // Output when the web server is successfully started.
            // case of cli
            let output_message = match language{
                Language::Japanese => format!("{running}: カレントディレクトリをhttpで配信しています。\n\
                http://localhost にアクセスすればブラウズができます。\n\n\
                終了する場合は、Ctrl + C を押すか、このウィンドを閉じてください。", running=styled_string.running),
                Language::English => format!("{running}: The current directory is served by http!!\n\
                You can browse by visiting http://localhost. \n\n\
                To exit, press Ctrl + C or close this window.", running=styled_string.running)
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