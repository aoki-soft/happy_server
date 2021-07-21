// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use super::server_core;
// use super::controller;
use super::*;

// Add color to console output
use colored::*;
use std::result::Result;


#[allow(dead_code)]
pub struct StyledString {
    pub error: String,
    pub note: String,
    pub running: String,
    pub finish: String
} impl StyledString {
    pub fn colored() -> Self {
        Self{
            error: "Error".red().bold().to_string(),
            note: "Note".blue().bold().to_string(),
            running: "Running".green().bold().to_string(),
            finish: "Finish".green().bold().to_string(),
        }
    }
    pub fn no_colored() -> Self {
        Self {
            error: "Error".to_string(),
            note: "Note".to_string(),
            running: "Running".to_string(),
            finish: "Finish".to_string(),
        }
    }
}


pub struct CliViewer{
    pub language: Language,
    pub style: StyledString
}

impl server_core::HappyServerViewer for CliViewer{
    fn start_happy_server(&self, hs_server: Result<Server,()>, hs_builder: HappyServerBuilder) -> Result<HappyServer, String> {
        match hs_server {
            Err(_) => {
                // Output when the web server fails to start.
                let output_message = match self.language{
                    Language::Japanese => format!("{error}: カレントディレクトリをhttpで配信できませんでした。", error=self.style.error),
                    Language::English => format!("{error}: The current directory could not be delivered via http.", error=self.style.error)
                };
                println!("{}", output_message);
                Err(output_message)
            },
            Ok(server) => {
                // Output when the web server is successfully started.
                // case of cli
                let url = match hs_builder.socket_addr.port() {
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

                Ok(server_core::HappyServer{
                    server: server,
                    hs_builder: hs_builder
                })
            }
        }
    }

    fn happy_server_stop(&self, _hs_stop: HappyServer) {
        match self.language {
            Language::Japanese => println!("{finish}: httpでの配信を終了しました。", finish=self.style.finish),
            Language::English => println!("{finish}: Distribution via http has been terminated.", finish=self.style.finish)
        }
    }
}

impl super::model::HappyServerModelViewer for CliViewer {
    fn to_happy_server_builder(&self, model: model::HappyServerModel) -> Result<crate::server_core::HappyServerBuilder, String> {
        match self.language {
            Language::Japanese => {
                // port convert string to u16
                let socketaddr: std::result::Result<u16, String> = match model.port {
                    Some(p) => match p.parse() {
                        Ok(p) => Ok(u16::into(p)),
                        Err(_) => Err(format!("{error}: コマンドライン引数のポート番号に、数値以外が入っていました。\n\
                        {note}: 引数には0~65535までの数値を入れることができます。\n\
                        {note}: 引数を入れなければ、デフォルトポート: {defalut_port}が利用します。"
                        , error=self.style.error, note=self.style.note, defalut_port=DEFAULT_HTTP_PORT))
                    },
                    None => Ok(u16::into(DEFAULT_HTTP_PORT))
                };
                let mut error_output = None;
                let port = match socketaddr {
                    Ok(p) => Some(p),
                    Err(e) => {
                        error_output = match error_output {
                            Some(previous) => Some(format!("{}\n{}", previous, e)),
                            None => Some(e)
                        };
                        None
                    }
                };

                match error_output {
                    Some(e) => {
                        println!("{}", e);
                        Err(e)
                    },
                    None => {
                        // output error if there is error
                        Ok(HappyServerBuilder{
                            socket_addr: std::net::SocketAddrV4::new(DEFAULT_IPV4_ADDR, port.unwrap())
                        })
                    }
                }
            },
            Language::English => {
                // port convert string to u16
                let socketaddr: std::result::Result<u16, String> = match model.port {
                    Some(p) => match p.parse() {
                        Ok(p) => Ok(u16::into(p)),
                        Err(_) => Err(format!("{error}: The port number in the command line argument contained a non-numeric value.\n\
                        {note}: The argument can be any number between 0 and 65535.\n\
                        {note}: If no argument is given, the default port: {defalut_port} will be used.", error=self.style.error, note=self.style.note, defalut_port=DEFAULT_HTTP_PORT))
                    },
                    None => Ok(u16::into(DEFAULT_HTTP_PORT))
                };
                let mut error_output = None;
                let port = match socketaddr {
                    Ok(p) => Some(p),
                    Err(e) => {
                        error_output = match error_output {
                            Some(previous) => Some(format!("{}\n{}", previous, e)),
                            None => Some(e)
                        };
                        None
                    }
                };
                
                match error_output {
                    Some(e) => {
                        println!("{}", e);
                        Err(e)
                    },
                    None => {
                        // output error if there is error
                        Ok(HappyServerBuilder{
                            socket_addr: std::net::SocketAddrV4::new(DEFAULT_IPV4_ADDR, port.unwrap())
                        })
                    }
                }
            }
        }
    }
}