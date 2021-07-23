// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use super::*;

use clap::*;
use super::model::HappyServerModel;
pub struct CliArgGetter{
    pub language: super::Language
}

impl CliArgGetter {
    pub fn get_arguments(&self) -> (Language, bool, Option<bool>, HappyServerModel){
        // accept command line arguments
        let mut args = app_from_crate!()
        .arg(Arg::with_name("port")
            .help(match self.language {
                Language::Japanese => "配信ポートの指定",
                Language::English => "Specify the distribution port",
            })
            .short("p")
            .long("port")
            .value_name("port_number")
            .takes_value(true))
        .arg(Arg::with_name("uri_prefix")
            .help(match self.language {
                Language::Japanese => "配信uriの指定 ... https://localhost/〇〇/index.html の〇〇の部分です。",
                Language::English => "Specifies the prefix of the delivery uri.",
            })
            .short("u")
            .long("uri_prefix")
            .value_name("uri prefix")
            .takes_value(true))
        .arg(Arg::with_name("distribution_dir")
            .help(match self.language {
                Language::Japanese => "配信ディレクトリの指定",
                Language::English => "Specify the distribution directory",
            })
            .short("d")
            .long("dist_dir")
            .value_name("distribution directory")
            .takes_value(true));
        args = match self.language {
            Language::Japanese => args.arg(Arg::with_name("english")
            .help("標準出力を英語にします")
            .short("e")
            .long("english")),
            Language::English => args.arg(Arg::with_name("japanese")
            .help("Prints output with Japanese")
            .short("j")
            .long("japanese"))
        };

        args = if cfg!(not(feature = "no_clipboard")) {
            args.arg(Arg::with_name("no_clipboard")
                .help(match self.language {
                    Language::Japanese => "クリップボード機能を使わない",
                    Language::English => "Do not use the clipboard function.",
                })
                .long("no_clipboard"))
        }else {args};

        args = if cfg!(feature = "no_color") {
            args.arg(Arg::with_name("color")
                .help( match self.language {
                    Language::Japanese => "標準出力に色を付けます(ターミナルによっては対応していません。)",
                    Language::English => "Add color to the standard output (not supported by some terminals)."
                })
                .short("c")
                .long("color"))
        } else {
            args.arg(Arg::with_name("no_color")
            .help( match self.language {
                Language::Japanese => "標準出力に色を付けません",
                Language::English => "Do not add color to standard output"
            })
            .long("no_color"))
        };

        // Parse the arguments
        let matches = args.get_matches();

        let color = if cfg!(feature = "no_color") {
            match matches.occurrences_of("color") {
                0 => false,
                _ => true,
            }
        } else {
            match matches.occurrences_of("no_color") {
                0 => true,
                _ => false,
            }
        };

        let using_clipboard = if cfg!(not(feature = "no_clipboard")) {
            match matches.occurrences_of("no_clipboard") {
                0 => Some(true),
                _ => Some(false)
            }
        }else {None};

        let language = match self.language {
            Language::Japanese => match matches.occurrences_of("english") {
                0 => Language::Japanese,
                _ => Language::English,
            },
            Language::English => match matches.occurrences_of("japanese") {
                0 => Language::English,
                _ => Language::Japanese
            }
        };

        // return value
        (language, color, using_clipboard, 
            HappyServerModel{
                port: match matches.value_of_lossy("port") { 
                    Some(port) => Some(port.to_string()),
                    None => None
                },
                distribution_dir: match matches.value_of_lossy("distribution_dir") {
                    Some(path) => Some(path.to_string()),
                    None => None
                },
                uri_prefix: match matches.value_of_lossy("uri_prefix"){
                    Some(uri_prefix) => Some(uri_prefix.to_string()),
                    None => None
                },
            }
        )
    }
}

