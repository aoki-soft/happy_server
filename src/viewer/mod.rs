// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use super::server_core;
use super::*;
use super::server_core::HappyServerBuilder;

// Add color to console output
use colored::*;

use std::num::ParseIntError;
use std::result::Result;
use std::io::Write;
use std::io;


const DEFAULT_IPV4_ADDR: Ipv4Addr  = Ipv4Addr::new(0, 0, 0, 0);
const DEFAULT_HTTP_PORT: u16  = 80;

#[allow(dead_code)]
pub struct StyledString {
    pub error: String,
    pub note: String,
    pub running: String,
    pub finish: String,
    pub copied: String,
} 

impl StyledString {
    pub fn colored() -> Self {
        Self{
            error: "Error".red().bold().to_string(),
            note: "Note".blue().bold().to_string(),
            running: "Running".green().bold().to_string(),
            finish: "Finish".green().bold().to_string(),
            copied: "Copied".green().bold().to_string(),
        }
    }
    pub fn no_colored() -> Self {
        Self {
            error: "Error".to_string(),
            note: "Note".to_string(),
            running: "Running".to_string(),
            finish: "Finish".to_string(),
            copied: "Copied".to_string(),
        }
    }
}


pub struct StreamViewer<T: Write>{
    pub language: Language,
    pub style: StyledString,
    pub writer: T,
    pub clipbood: Option<bool>,
}

#[cfg(not(feature="no_clipboard"))]
fn set_clipboard(set_string: String) -> Result<(), Box<dyn std::error::Error>> {
    use clipboard::ClipboardContext;
    use clipboard::ClipboardProvider;
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(set_string)
}
#[cfg(not(feature="no_clipboard"))]
fn set_url_to_clipboard<T: Write>(viewer: &StreamViewer<T>, url: String, clipboard_result_string: &mut String) {
    match viewer.clipbood {
        Some(true) => {
            let clipboard_result = set_clipboard(url);
            
            match clipboard_result {
                Ok(_) => {*clipboard_result_string = match viewer.language {
                    Language::Japanese => format!("{copied}: クリップボードにURLをコピーしました!!\n", copied= viewer.style.copied),
                    Language::English => format!("{copied}: copied the URL to the clipboard!\n", copied= viewer.style.copied)
                }},
                Err(_) => ()
            }
        },
        _ => ()
    }
}
#[cfg(feature="no_clipboard")]
fn set_url_to_clipboard<T: Write>(_viewer: &StreamViewer<T>, _url: String, _clipboard_result_string: &mut String) {}

impl<T: Write> server_core::HappyServerViewer for StreamViewer<T> {
    fn start_happy_server(&mut self, hs_server: Result<Server,()>, hs_builder: HappyServerBuilder) -> io::Result<Result<HappyServer, String>> {
        match hs_server {
            Err(_) => {
                // Output when the web server fails to start.
                let output_message = match self.language{
                    Language::Japanese => format!("{error}: カレントディレクトリをhttpで配信できませんでした。\n", error=self.style.error),
                    Language::English => format!("{error}: The current directory could not be delivered via http.\n", error=self.style.error)
                };
                self.writer.write_all(output_message.as_bytes())?;
                Ok(Err(output_message))
            },
            Ok(server) => {
                // Output when the web server is successfully started.
                let url = match hs_builder.socket_addr.port() {
                    DEFAULT_HTTP_PORT => format!("http://localhost"),
                    num => format!("http://localhost:{}",num)
                };
                // Paste url to clipboard
                let mut clipboard_result_string = String::new();
                set_url_to_clipboard(&self, url.clone(), &mut clipboard_result_string);

                let output_message = match self.language{
                    Language::Japanese => format!("\
                    {running}: カレントディレクトリをhttpで配信しています。\n\
                    {url} にアクセスすればブラウズができます。\n\n\
                    {clipboard_result_string}\
                    終了する場合は、Ctrl + C を押すか、このウィンドを閉じてください。\n"
                    , url=url, running=self.style.running, clipboard_result_string=clipboard_result_string),
                    Language::English => format!("\
                    {running}: The current directory is served by http!!\n\
                    You can browse by visiting {url}\n\n\
                    {clipboard_result_string}\
                    To exit, press Ctrl + C or close this window.\n"
                    , url=url, running=self.style.running, clipboard_result_string=clipboard_result_string)
                };
                self.writer.write_all(output_message.as_bytes())?;

                Ok(Ok(server_core::HappyServer{
                    server: server,
                    hs_builder: hs_builder
                }))
            }
        }
    }

    fn happy_server_stop(&mut self, _hs_stop: HappyServer) -> io::Result<()> {
        let output_result = match self.language {
            Language::Japanese => format!("{finish}: httpでの配信を終了しました。\n", finish=self.style.finish),
            Language::English => format!("{finish}: Distribution has been terminated.\n", finish=self.style.finish)
        };
        self.writer.write_all(output_result.as_bytes())?;
        Ok(())
    }
}

impl<T: Write> super::model::HappyServerModelViewer for StreamViewer<T> {
    fn to_happy_server_builder(&mut self, port: Result<u16, ParseIntError>) -> io::Result<Result<HappyServerBuilder, String>> {
        // port convert string to u16
        let port: Result<u16, String> = match port {
            Ok(p) => Ok(p),
            Err(_) => Err(
                match self.language {
                    Language::Japanese => format!("\
                        {error}: コマンドライン引数のポート番号に、数値以外が入っていました。\n\
                        {note}: 引数には0~65535までの数値を入れることができます。\n\
                        {note}: オプションを入れなければ、{defalut_port}番ポートを使用します。\n"
                        , error=self.style.error, note=self.style.note, defalut_port=DEFAULT_HTTP_PORT),
                    Language::English => format!("\
                        {error}: The port number in the command line argument contained a non-numeric value.\n\
                        {note}: The argument can be any number between 0 and 65535.\n\
                        {note}: If no argument is given, the default port: {defalut_port} will be used.\n"
                        , error=self.style.error, note=self.style.note, defalut_port=DEFAULT_HTTP_PORT)
                }
            )
        };
        let mut error_output = None;
        let port = match port {
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
                self.writer.write_all(e.as_bytes())?;
                Ok(Err(e))
            },
            None => {
                Ok(Ok(HappyServerBuilder{
                    socket_addr: std::net::SocketAddrV4::new(DEFAULT_IPV4_ADDR, port.unwrap())
                }))
            }
        }
    }
}