// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use actix_files;
use actix_web::{App, HttpServer, dev::Server};
use std::net::SocketAddrV4;

pub struct HappyServer(pub std::io::Result<Server>, pub SocketAddrV4);
impl HappyServer {
    /// # start web server
    /// # Returns
    /// * webserver handler or err
    async fn start_server(socket_addr: &SocketAddrV4) -> std::io::Result<Server> {
        Ok(HttpServer::new(|| {
            App::new().service(actix_files::Files::new("/", ".").show_files_listing())
        })
        .bind(&socket_addr)?
        .run())
    }
    pub async fn start(socket_addr: SocketAddrV4) -> Self {
        HappyServer(HappyServer::start_server(&socket_addr).await, socket_addr)
    }
}
