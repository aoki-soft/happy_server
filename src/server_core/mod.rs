// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use actix_files;
use actix_web::{App, HttpServer, dev::Server};
pub struct HappyServer(pub std::io::Result<Server>);
impl HappyServer {
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
    pub async fn start() -> Self {
        HappyServer(HappyServer::start_server().await)
    }
}
