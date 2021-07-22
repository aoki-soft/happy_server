// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  
mod server_core;
mod viewer;
mod controller;
mod model;

use viewer::*;
use actix_web::dev::Server;
use server_core::HappyServer;
use std::net::Ipv4Addr;

// Compile-time defaults
#[cfg(any(feature = "japanese",not(feature = "english")))]
const COMPILE_TIME_DEFAULT_LANGUAGE:Language = Language::Japanese;
#[cfg(feature = "english")]
const COMPILE_TIME_DEFAULT_LANGUAGE:Language = Language::English;

/// Language settings
#[allow(dead_code)]
pub enum Language {
    Japanese,
    English
}

/// # application entry point
/// Note: Although it is an async function, it is converted to a normal function signature by the #[actix_web::main] attribute.
#[actix_web::main]
async fn main() {
    // get default settings
    // language setting
    let language = COMPILE_TIME_DEFAULT_LANGUAGE;

    // setup cli arguments getter
    let cli_arg_getter = controller::CliArgGetter{language};
    // get cli arguments
    let (language, color, happy_server_model) = cli_arg_getter.get_arguments();
    // setup cli style strings
    let style = if color {StyledString::colored()} else {StyledString::no_colored()};
    // setup app viewer
    let mut viewer = StreamViewer{language, style, writer: std::io::stdout(), clipbood: Some(true)};
    // convert model to server builder, then output with viewer
    let server_builder = happy_server_model.to_happy_server_builder(&mut viewer).unwrap_or_else(|_op|{
        // if the output of the viewer is not successful
        std::process::exit(1)
    }).unwrap_or_else(|_op|{
        // if there is error cli argument
        std::process::exit(0)
    });

    // run happy server and output server start result
    let server = server_builder.start_server(&mut viewer).await.unwrap_or_else(|_op|{
        // if the output of the viewer is not successful
        std::process::exit(1)
    }).unwrap_or_else(|_| {
        // if happy server is not working
        std::process::exit(0)
    });

    // wait finish signal
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        tokio::signal::ctrl_c().await.unwrap();
    });
    
    // stop server and output stop server result
    server.stop(&mut viewer).await.unwrap_or_else(|_op|{
        // if the output of the viewer is not successful
        std::process::exit(1)
    });
}





#[cfg(test)]
mod test {
    // use actix_web::client::Client;
    // use std::path::PathBuf;
    // use std::thread;
    // use std::time::Duration;
    // use std::sync::Arc;
    // use std::fs::File;
    // use std::io::Write;

    // #[test]
    // fn get_file() {

    //     let test_file_text = Arc::new("Hello World!! \n 日本語です。".to_owned());
    //     // let mut test_file_path = std::env::current_exe().unwrap().parent().unwrap().to_owned();
    //     let mut test_file_path = std::env::current_dir().unwrap();
    //     let test_file_name = "test_file.txt";
    //     test_file_path.push(test_file_name);
    //     let mut test_file = File::create(test_file_path.clone()).unwrap();
    //     test_file.write(test_file_text.as_bytes()).unwrap();


    //     let main_handle = thread::spawn(|| {
    //         let _ = super::main();
    //     });

    //     let test_handle = thread::spawn(|| {test_request(test_file_text, test_file_path)});
    //     test_handle.join().unwrap();
    //     main_handle.join().unwrap();
        
    // }

    // #[actix_web::main]
    // async fn test_request(test_file_text: Arc<String>, test_file_path: PathBuf) {
    //     thread::sleep(Duration::from_millis(10000));

    //     // request
    //     let client = Client::default();
    //     let mut res = client.get("http://localhost/test_file.txt") // <- Create request builder
    //         .header("User-Agent", "Actix-web")
    //         .send()                             // <- Send http request
    //         .await.unwrap();
        
    //     let res = res.body();
    //     let res = res.await.unwrap();

    //     assert_eq!(res, *test_file_text.as_bytes());
    //     std::fs::remove_file(test_file_path).unwrap();
    //     std::process::exit(0);
    // }

    // #[actix_web::test]
    // async fn test_index_ok() {
    //     let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
    //     let resp = index(req).await;
    //     assert_eq!(resp.status(), http::StatusCode::OK);
    // }


    #[test]
    fn test_for_test(){
        // ok
    }
}