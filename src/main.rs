// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  
mod server_core;

use std::io;
use actix_web::dev::Server;
// Add color to console output
use colored::*;
use server_core::HappyServer;
use std::net::{Ipv4Addr, SocketAddrV4};

// Compile-time defaults
#[cfg(any(feature = "japanese",not(feature = "english")))]
const COMPILE_TIME_DEFAULT_LANGUAGE:Language = Language::Japanese;
#[cfg(feature = "english")]
const COMPILE_TIME_DEFAULT_LANGUAGE:Language = Language::English;

#[cfg(not(feature = "no_color"))]
const COMPILE_TIME_DEFAULT_COLOR: bool = true;
#[cfg(feature = "no_color")]
const COMPILE_TIME_DEFAULT_COLOR: bool = false;

const DEFAULT_IPV4_ADDR: Ipv4Addr  = Ipv4Addr::new(0, 0, 0, 0);
const DEFAULT_HTTP_PORT: u16  = 80;
// const DEFAULT_PROTOCOL: Protocol<()> = Protocol::Http(());

// enum Protocol<T> {
//     Http(T),
//     // Https(T)
// }

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

/// Viewer enums
enum Viewer {
    Cli(CliViewer)
    // TODO: WebApi
    // TODO: Windows Service
}

struct CliViewer{
    language: Language,
    style: StyledString
}

impl Viewer {
    fn output_reslut(self, server: HappyServer) -> Server{
        match self {
            Viewer::Cli(cli) => cli.result_server(server)
            // TODO: WebApi
            // TODO: Windows Service
        }
    }
}

// cli output
impl CliViewer {
    fn result_server(self, server: HappyServer) -> Server {
        match server.0 {
            Err(_) => {
                // Output when the web server fails to start.
                let output_message = match self.language{
                    Language::Japanese => format!("{error}: カレントディレクトリをhttpで配信できませんでした。", error=self.style.error),
                    Language::English => format!("{error}: The current directory could not be delivered via http.", error=self.style.error)
                };
                println!("{}", output_message);
                std::process::exit(0);
            },
            Ok(web_server) => {
                // Output when the web server is successfully started.
                // case of cli
                let url = match server.1.port() {
                    DEFAULT_HTTP_PORT => format!("http://localhost"),
                    num => format!("http://localhost:{}",num)
                };

                let output_message = match self.language{
                    Language::Japanese => format!("{running}: カレントディレクトリをhttpで配信しています。\n\
                    {url} にアクセスすればブラウズができます。\n\n\
                    終了する場合は、Ctrl + C を押すか、このウィンドを閉じてください。", url=url, running=self.style.running),
                    Language::English => format!("{running}: The current directory is served by http!!\n\
                    You can browse by visiting {url}. \n\n\
                    To exit, press Ctrl + C or close this window.",url=url, running=self.style.running)
                };
                println!("{}", output_message);

                web_server
            }
        }
    }
}


/// # application entry point
/// Note: Although it is an async function, it is converted to a normal function signature by the #[actix_web::main] attribute.
#[actix_web::main]
async fn main() -> io::Result<()> {
    // get default settings
    // language setting
    let language = COMPILE_TIME_DEFAULT_LANGUAGE;
    // cli color setting  ex) true = colored , false = no_colored
    let cli_color = COMPILE_TIME_DEFAULT_COLOR;
    // cli style strings
    let style = if cli_color {
        StyledString::colored()
    } else {
        StyledString::no_colored()
    };

    let ipv4_addr = DEFAULT_IPV4_ADDR;
    let port = DEFAULT_HTTP_PORT;



    // viewer setup
    let viewer = Viewer::Cli(CliViewer{language, style});
    // TODO: WebApi
    // TODO: Windows Service

    // TODO: Determine what you want to do.
    // if install this app, ...

    // if start up server
    // TODO: setup web_server
    // TODO: output result of setup web_server

    let socket_addr = SocketAddrV4::new(ipv4_addr, port);

    // start web server
    let web_server = HappyServer::start(socket_addr).await;
    // Output the result of the Happy Server startup.
    let web_server = viewer.output_reslut(web_server);

    // TODO: Finish Server

    // Wait for it to finish.
    web_server.await
}
