// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use actix_files;
use actix_web::{App, HttpServer, dev::Server};
use std::net::SocketAddrV4;
use std::result::Result;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;
use std::io::BufRead;

pub struct HappyServerBuilder{
    pub socket_addr: SocketAddrV4,
    pub distribution_dir: PathBuf,
    pub uri_prefix: String,
    pub ssl: Option<Ssl>,
}
pub struct Ssl {
    pub server_crt: Box<dyn BufRead>,
    pub server_key: Box<dyn BufRead>,
}

#[cfg(feature="no_ssl")]
fn start_server(builder: HappyServer) -> Result<Server, ()> {
    match builder.ssl.as_mut() {
        Some(ssl) => {
            Err(())
        }
        None=> {
            let distribution_dir = Arc::new(builder.distribution_dir.clone());
            let uri_prefix = Arc::new(builder.uri_prefix.clone());
            let server = Ok(HttpServer::new(move|| {
                App::new().service(actix_files::Files::new(&uri_prefix, &*distribution_dir).show_files_listing())
            })
            .bind(&builder.socket_addr).unwrap()
            .run());
            server
        }
    }
}
#[cfg(not(feature="no_ssl"))]
fn start_server(builder: &mut HappyServerBuilder) -> Result<Server, ()> {
    match builder.ssl.as_mut() {
        Some(ssl) => {
            let certs = rustls::internal::pemfile::certs(&mut ssl.server_crt).unwrap();
            let mut keys = rustls::internal::pemfile::pkcs8_private_keys(&mut ssl.server_key).unwrap();
            let mut cfg = rustls::ServerConfig::new(rustls::NoClientAuth::new());
            cfg.set_single_cert(certs, keys.remove(0)).unwrap();
            cfg.set_protocols(&[b"h2".to_vec(), b"http/1.1".to_vec()]);

            let distribution_dir = Arc::new(builder.distribution_dir.clone());
            let uri_prefix = Arc::new(builder.uri_prefix.clone());

            let server = Ok(HttpServer::new(move|| {
                App::new().service(actix_files::Files::new(&uri_prefix, &*distribution_dir).show_files_listing())
            })
            .bind_rustls(&builder.socket_addr, cfg).unwrap()
            .run());
            server
        }
        None=> {
            let distribution_dir = Arc::new(builder.distribution_dir.clone());
            let uri_prefix = Arc::new(builder.uri_prefix.clone());
            let server = Ok(HttpServer::new(move|| {
                App::new().service(actix_files::Files::new(&uri_prefix, &*distribution_dir).show_files_listing())
            })
            .bind(&builder.socket_addr).unwrap()
            .run());
            server
        }
    }
}

impl HappyServerBuilder {
    pub async fn start_server(&mut self) -> Result<Server,()> {
        start_server(self)
    }
}


pub trait HappyServerViewer {
    fn output_start_server(&mut self, hs_server: &Result<Server, ()>, hs_builder: &HappyServerBuilder) -> io::Result<()>;
    fn output_server_stop(&mut self, hs_server: &HappyServer) -> io::Result<()>;
}


pub struct HappyServer {
    pub server: Server,
    pub hs_builder: HappyServerBuilder
}

#[allow(dead_code)]
impl HappyServer {
    pub async fn stop(& self) {
        self.server.stop(false).await;
    }
    pub async fn awaiting(self) -> std::io::Result<()>{
        self.server.await
    }
}