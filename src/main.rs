extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use] extern crate tera;

#[cfg(debug_assertions)] use std::env;
use actix_web::server;

pub mod app;

fn main() {
    #[cfg(debug_assertions)] {
        env::set_var("RUST_LOG", "actix_web=debug");
        env::set_var("RUST_BACKTRACE", "0");
        env_logger::init();
    }
    let sys = actix::System::new("ultimate");
    let _addr = server::new(|| app::create_app())
        .bind("127.0.0.1:8000")
        .expect("Can not bind to 127.0.0.1:8000")
        .start();
    let _ = sys.run();
}
