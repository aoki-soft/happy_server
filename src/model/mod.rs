// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use super::server_core::HappyServerBuilder;
use std::io;
// use std::net::Ipv4Addr;
use std::num::ParseIntError;

// const DEFAULT_IPV4_ADDR: Ipv4Addr  = Ipv4Addr::new(0, 0, 0, 0);
const DEFAULT_HTTP_PORT: u16  = 80;

pub struct HappyServerModel {
    pub port: Option<String>
}

pub trait HappyServerModelViewer {
    fn to_happy_server_builder(&mut self, port: Result<u16, ParseIntError>) -> io::Result<Result<HappyServerBuilder, String>>;
}

impl HappyServerModel {
    pub fn to_happy_server_builder(self, viewer: &mut impl HappyServerModelViewer) -> io::Result<Result<HappyServerBuilder, String>> {
        let port = match self.port {
            Some(port) => port.parse::<u16>(),
            None => Ok(u16::into(DEFAULT_HTTP_PORT))
        };
        viewer.to_happy_server_builder(port)
    }
}