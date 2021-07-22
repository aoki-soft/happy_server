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
        let args = match self.language {
            Language::Japanese => {
                // accept command line arguments
                let mut args = app_from_crate!()
                .arg(Arg::with_name("port")
                    .help("配信ポートの指定")
                    .short("p")
                    .long("port")
                    .value_name("port_number")
                    .takes_value(true))
                .arg(Arg::with_name("english")
                    .help("標準出力を英語にします")
                    .short("e")
                    .long("english"));

                    args = if cfg!(not(feature = "no_clipboard")) {
                        args.arg(Arg::with_name("no_clipboard")
                            .help("クリップボード機能を使わない")
                            .long("no_clipboard"))
                    }else {args};

                args = if cfg!(feature = "no_color") {
                    args.arg(Arg::with_name("color")
                        .help("標準出力に色を付けます(ターミナルによっては対応していません。)")
                        .short("c")
                        .long("color"))
                } else {
                    args.arg(Arg::with_name("no_color")
                    .help("標準出力に色を付けません")
                    .long("no_color"))
                };
                args
            },
            Language::English => {
                // accept command line arguments
                let mut args = app_from_crate!()
                    .arg(Arg::with_name("port")
                        .help("Specify the distribution port")
                        .short("p")
                        .long("port")
                        .value_name("port_number")
                        .takes_value(true))
                    .arg(Arg::with_name("japanese")
                        .help("Prints output with Japanese")
                        .short("j")
                        .long("japanese"));

                args = if cfg!(not(feature = "no_clipboard")) {
                    args.arg(Arg::with_name("no_clipboard")
                        .help("Do not use the clipboard function.")
                        .long("no_clipboard"))
                }else {args};

                args = if cfg!(feature = "no_color") {
                    args.arg(Arg::with_name("color")
                        .help("Add color to the standard output (not supported by some terminals).")
                        .short("c")
                        .long("color"))
                } else {
                    args.arg(Arg::with_name("no_color")
                    .help("Do not add color to standard output")
                    .long("no_color"))
                };
                args
            }
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
                    Some(p) => Some(p.to_string()),
                    None => None
                }
            }
        )
    }
}

