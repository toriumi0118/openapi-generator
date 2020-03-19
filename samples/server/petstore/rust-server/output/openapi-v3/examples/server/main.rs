//! Main binary entry point for openapi_v3 implementation.

#![allow(missing_docs)]

// Imports required by this file.
// extern crate <name of this crate>;
extern crate openapi_v3;
extern crate clap;
extern crate env_logger;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate swagger;

// Imports required by server library.
// extern crate openapi_v3;
extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate futures;
// extern crate swagger;
extern crate tokio;
extern crate uuid;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
extern crate openssl;
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
extern crate tokio_openssl;

use clap::{App, Arg};

mod server;


/// Create custom server, wire it to the autogenerated router,
/// and pass it to the web server.
fn main() {
    env_logger::init();

    let matches = App::new("server")
        .arg(Arg::with_name("https")
            .long("https")
            .help("Whether to use HTTPS or not"))
        .get_matches();

    let addr = "127.0.0.1:80";

    hyper::rt::run(server::create(addr, matches.is_present("https")));
}
