// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use actix_files;
use actix_web::{App, HttpServer, dev::Server};
use std::net::SocketAddrV4;
use std::result::Result;

pub struct HappyServerBuilder{
    pub socket_addr: SocketAddrV4
}

impl HappyServerBuilder {
    pub async fn start_server(self, viewer: &mut impl HappyServerViewer) -> Result<HappyServer, String> {
        let server = Ok(HttpServer::new(|| {
            App::new().service(actix_files::Files::new("/", ".").show_files_listing())
        })
        .bind(&self.socket_addr).unwrap()
        .run());
        viewer.start_happy_server(server, self)
    }
}


pub trait HappyServerViewer {
    fn happy_server_stop(&mut self, hs_server: HappyServer);
    fn start_happy_server(&mut self, hs_server: Result<Server,()>, hs_builder: HappyServerBuilder) -> Result<HappyServer,String>;
}


pub struct HappyServer {
    pub server: Server,
    pub hs_builder: HappyServerBuilder
}

#[allow(dead_code)]
impl HappyServer {
    pub async fn stop(self, viewer: &mut impl HappyServerViewer){
        self.server.stop(false).await;
        viewer.happy_server_stop(self);
    }
    pub async fn awaiting(self) -> std::io::Result<()>{
        self.server.await
    }
}