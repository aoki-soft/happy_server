// Copyright (c) 2021 Daichi Aoki  
// Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  

use super::*;

use clap::*;
use super::model::HappyServerModel;
pub struct CliArgGetter{
    pub language: super::Language
}

impl CliArgGetter {
    pub fn get_arguments(&self) -> (Language, HappyServerModel){
        match self.language {
            Language::Japanese => {
                // accept command line arguments
                let args = app_from_crate!()
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
                let matches = args.get_matches();
                
                // return value
                (match matches.occurrences_of("english") {
                        0 => Language::Japanese,
                        _ => Language::English,
                    },
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
                let args = app_from_crate!()
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
                let matches = args.get_matches();

                // return value
                (match matches.occurrences_of("japanese") {
                        0 => Language::English,
                        _ => Language::Japanese
                    },
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

