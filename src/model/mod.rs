// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use super::server_core::HappyServerBuilder;
use std::io;

pub struct HappyServerModel {
    pub port: Option<String>
}

pub trait HappyServerModelViewer {
    fn to_happy_server_builder(&mut self, model: HappyServerModel) -> io::Result<Result<HappyServerBuilder, String>>;
}

impl HappyServerModel {
    pub fn to_happy_server_builder(self, viewer: &mut impl HappyServerModelViewer) -> io::Result<Result<HappyServerBuilder, String>> {
        viewer.to_happy_server_builder(self)
    }
}