// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use super::*;

use clap::*;
use super::model::HappyServerModel;
pub struct CliArgGetter{
    pub language: super::Language
}

impl CliArgGetter {
    pub fn get_arguments(&self) -> (Language, bool, HappyServerModel){
        match self.language {
            Language::Japanese => {
                // accept command line arguments
                let mut args = app_from_crate!()
                .after_help("\
                    \tカレントディレクトリを即座に配信します。")
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

                // return value
                (match matches.occurrences_of("english") {
                        0 => Language::Japanese,
                        _ => Language::English,
                    },
                    color,
                    HappyServerModel{
                        port: match matches.value_of_lossy("port") { 
                            Some(p) => Some(p.to_string()),
                            None => None
                        }
                    }
                )
            },
            Language::English => {
                // accept command line arguments
                let mut args = app_from_crate!()
                    .after_help("\
                        \tDeliver the current directory immediately.")
                    .arg(Arg::with_name("port")
                        .help("Specify the distribution port")
                        .short("p")
                        .long("port")
                        .value_name("port_number")
                        .takes_value(true))
                    .arg(Arg::with_name("japanese")
                        .help("Standard output in Japanese")
                        .short("j")
                        .long("japanese"));

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
                // return value
                (match matches.occurrences_of("japanese") {
                        0 => Language::English,
                        _ => Language::Japanese
                    },
                    color,
                    HappyServerModel{
                        port: match matches.value_of_lossy("port") { 
                            Some(p) => Some(p.to_string()),
                            None => None
                        }
                    }
                )
            }
        }
    }
}

