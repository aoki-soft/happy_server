// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use super::server_core::HappyServerBuilder;
use std::io;
use std::net::Ipv4Addr;
use std::num::ParseIntError;

const DEFAULT_IPV4_ADDR: Ipv4Addr  = Ipv4Addr::new(0, 0, 0, 0);
const DEFAULT_HTTP_PORT: u16  = 80;

pub enum ParameterSource<T> {
    Default(T),
    CliArg(T)
}
impl<T> ParameterSource<T> {
    fn get_contents(self) -> T {
        match self {
            Self::Default(contents) => contents,
            Self::CliArg(contents) => contents,
        }
    }
}

pub struct HappyServerModel {
    pub port: Option<String>,
    pub distribution_dir: Option<String>,
}

use std::path::PathBuf;
use std::str::FromStr;

pub trait HappyServerModelViewer {
    fn to_happy_server_builder(&mut self, port: & Result<u16, ParseIntError>, distribution_dir: &ParameterSource<Result<PathBuf, ()>>) -> io::Result<()>;
}

impl HappyServerModel {
    /// # Form incomplete parameters.
    /// Process the incomplete set of server parameters by putting in default values, etc., so that the server can be started.
    /// If the values are not enough, use the viewer in the argument to display errors, etc.
    pub fn to_happy_server_builder(self, viewer: &mut impl HappyServerModelViewer) -> (io::Result<()>, Result<HappyServerBuilder, ()>) {

        let port = match self.port {
            Some(port) => port.parse::<u16>(),
            None => Ok(u16::into(DEFAULT_HTTP_PORT))
        };
        
        let distribution_dir = match self.distribution_dir {
            Some(path) => match PathBuf::from_str(&path){
                Ok(path) => ParameterSource::CliArg(Ok(path)),
                Err(_) => ParameterSource::CliArg(Err(()))
            }
            None => match std::env::current_dir() {
                Ok(path) => ParameterSource::Default(Ok(path)),
                Err(_) => ParameterSource::Default(Err(()))
            }
        };

        let output_result = viewer.to_happy_server_builder(&port, &distribution_dir);

        let happy_server_builder_result = 
            match (port, distribution_dir.get_contents()) {
                (Ok(port), Ok(path)) => Ok(HappyServerBuilder{
                    socket_addr: std::net::SocketAddrV4::new(DEFAULT_IPV4_ADDR, port),
                    distribution_dir: path,
                    uri_prefix: "/".to_string(),
                }),
                _ => Err(())
        };

        (output_result, happy_server_builder_result)
    }
}