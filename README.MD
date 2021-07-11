# Happy Server

Happy Server is a simple Web Server written in Rust.

More features will be added in the future.
If you have any suggestions for improvement, please create an issue and let us know.

This software is written under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE).  

### Build

It needs to be connected to Rust's tool chain and to the Internet.

1. Download the source code from this repository.  

2. Start the build by executing the following command in the root directory of the repository.
```
cargo build --release --features english
```
When the build is finished, `./target/release/happy_server.exe` will be generated.

#### features
The default can be changed at compile time using the `--feature [value]` flag.

##### Language
If you take the english option, the cli output will be in Japanese.
```
cargo build --release
```

##### cli output color
You can add the no_color option to make the default cli output without color.  
If you don't have Windows Terminal installed, it is better to set no_color as default to prevent cli display clutter.
```
cargo build --features english --features no_color
```
___
Created on July 10, 2021  
Copyright (c) 2021 Daichi Aoki  
Released under the [MIT license](https://github.com/blz-soft/happy_server/blob/main/LICENSE)  