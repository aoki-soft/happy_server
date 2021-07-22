// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

pub struct HappyServerModel {
    pub port: Option<String>
}

pub trait HappyServerModelViewer {
    fn to_happy_server_builder(&mut self, model: HappyServerModel) -> Result<super::server_core::HappyServerBuilder, String>;
}

impl HappyServerModel {
    pub fn to_happy_server_builder(self, viewer: &mut impl HappyServerModelViewer) -> Result<super::server_core::HappyServerBuilder, String> {
        viewer.to_happy_server_builder(self)
    }
}