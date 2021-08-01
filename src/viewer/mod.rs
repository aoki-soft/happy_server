// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use super::server_core;
use super::*;
use super::server_core::HappyServerBuilder;

// Add color to console output
use colored::*;

use std::result::Result;
use std::io::Write;
use std::io;

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
    pub using_clipboard: Option<bool>,
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
    match viewer.using_clipboard {
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
    fn output_start_server(&mut self, hs_server: &Result<Server, ()>, hs_builder: &HappyServerBuilder) -> io::Result<()> {
        match hs_server {
            Err(_) => {
                // Output when the web server fails to start.
                let output_message = match self.language{
                    Language::Japanese => format!("{error}: httpによる配信を開始できませんでした。。\n", error=self.style.error),
                    Language::English => format!("{error}: Could not start delivery via http.\n", error=self.style.error)
                };
                self.writer.write_all(output_message.as_bytes())?;
                Ok(())
            },
            Ok(_server) => {
                // Output when the web server is successfully started.
                let url = match hs_builder.socket_addr.port() {
                    DEFAULT_HTTP_PORT => format!("http://localhost/{}", hs_builder.uri_prefix),
                    num => format!("http://localhost:{}/{}",num, hs_builder.uri_prefix)
                };
                // Paste url to clipboard
                let mut clipboard_result_string = String::new();
                set_url_to_clipboard(&self, url.clone(), &mut clipboard_result_string);

                let output_message = match self.language{
                    Language::Japanese => format!("\
                    {running}: httpでの配信を開始しました。\n\
                    {url} からブラウズできます。\n\n\
                    {clipboard_result_string}\
                    終了する場合は、Ctrl + C を押すか、このウィンドを閉じてください。\n"
                    , url=url, running=self.style.running, clipboard_result_string=clipboard_result_string),
                    Language::English => format!("\
                    {running}: Distribution via http is now available!!\n\
                    You can browse by visiting {url}\n\n\
                    {clipboard_result_string}\
                    To exit, press Ctrl + C or close this window.\n"
                    , url=url, running=self.style.running, clipboard_result_string=clipboard_result_string)
                };
                self.writer.write_all(output_message.as_bytes())?;
                Ok(())
            }
        }
    }

    fn output_server_stop(&mut self, _hs_stop: &HappyServer) -> io::Result<()> {
        let output_result = match self.language {
            Language::Japanese => format!("{finish}: httpでの配信を終了しました。\n", finish=self.style.finish),
            Language::English => format!("{finish}: Distribution has been terminated.\n", finish=self.style.finish)
        };
        self.writer.write_all(output_result.as_bytes())?;
        Ok(())
    }
}

use super::model::ParameterSource;
use super::model::HappyServerPreModel;

impl<T: Write> super::model::HappyServerModelViewer for StreamViewer<T> {
    fn output_server_pre_model(&mut self, model: & HappyServerPreModel) -> io::Result<()> {
        let mut error_output = None;
        
        match model.port {
            Err(_) => {
                let err_message = match self.language {
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
                };
                error_output = match error_output {
                    Some(prev_error) => Some(format!("{}{}",prev_error, err_message)),
                    None => Some(err_message)
                };
            },
            _ => ()
        }

        match &model.distribution_dir {
            ParameterSource::Default(path) => match path {
                Err(_) => {
                    let err_message = match self.language {
                        Language::Japanese => format!("{error}: カレントディレクトリを特定できませんでした。\n\
                            {note}: このアプリが環境変数にアクセスできないようになっている可能性があります。\n\
                            {note}: 可能であれば、配信するディレクトリを変更してアプリを実行してください。\n"
                            , error=self.style.error, note=self.style.note),
                        Language::English => format!("{error}: Could not locate the current directory.\n\
                            {note}: This app may not have access to environment variables. \n\
                            {note}: If possible, please change the directory to be delivered and run the app.\n"
                            , error=self.style.error, note=self.style.note),
                    };
                    error_output = match error_output {
                        Some(prev_error) => Some(format!("{}{}",prev_error, err_message)),
                        None => Some(err_message)
                    };
                },
                _ => ()
            },
            ParameterSource::CliArg(path) => match path {
                Err(_) => {
                    let err_message = match self.language {
                        Language::Japanese => format!("{error}: コマンドライン引数の「配信ディレクトリパス」のフォーマットが不正です。\n", error=self.style.error),
                        Language::English => format!("{error}: The format of the command line argument \"delivery directory path\" is invalid.\n", error=self.style.error),
                    };
                    error_output = match error_output {
                        Some(prev_error) => Some(format!("{}{}",prev_error, err_message)),
                        None => Some(err_message)
                    };
                },
                _ => ()
            }
        };

        match &model.uri_prefix {
            ParameterSource::CliArg(uri_prefix) => match uri_prefix {
                Ok(_) => (),
                Err(_) => {
                    let err_message = match self.language {
                        Language::Japanese => format!("{error}: コマンドライン引数の「配信uriの指定」の値が不正です。\n\
                        {note}: URIに使用できる文字を使ってください。\n\
                        {note}: 最初に\"/\"を入れられません。また、\"//\"を入れることができません。\n", error=self.style.error, note=self.style.note),
                        Language::English => format!("{error}: The value of the URI setting in the command line argument is invalid.\n\
                        {note}: Please use characters that can be used for URIs.\n\
                        {note}: You can't put \"/\" at the beginning. You also cannot enter \"//\"\n", error=self.style.error, note=self.style.note),
                    };
                    error_output = match error_output {
                        Some(prev_error) => Some(format!("{}{}",prev_error, err_message)),
                        None => Some(err_message)
                    };
                }
            },
            ParameterSource::Default(_) => (),
        };

        // Print any error messages.
        match error_output {
            Some(e) => self.writer.write_all(e.as_bytes()),
            None => Ok(())
        }
    }
}