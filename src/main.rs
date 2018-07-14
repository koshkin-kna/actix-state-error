extern crate actix;
extern crate actix_web;
extern crate env_logger;
#[macro_use]
extern crate tera;

use actix_web::http::{ContentEncoding, Method, NormalizePath, StatusCode};
use actix_web::{fs, middleware, pred, server, App, HttpRequest, HttpResponse};
use std::env;

pub struct AppState {
    template: tera::Tera,
}


fn index(_req: HttpRequest<AppState>) -> HttpResponse {

    let mut m = &_req.state().template;
   // m.full_reload().unwrap();

    let mut context = tera::Context::new();
    context.add("vat_rate", &0.20);
    let s = m
        .render("admin/login.html", &context)
        .unwrap();

    //let s = tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/**/*")).unwrap().render("index/index2.html", &context).unwrap();
    HttpResponse::Ok()
        .content_encoding(ContentEncoding::Gzip)
        .content_type("text/html; charset=utf-8")
        .body(s)
}

fn p404(_req: HttpRequest<AppState>) -> HttpResponse {
    let mut context = tera::Context::new();
    context.add("vat_rate", &0.20);
    let s = _req.state().template.render("404.html", &context).unwrap();
    HttpResponse::build(StatusCode::NOT_FOUND)
        .content_encoding(ContentEncoding::Gzip)
        .content_type("text/html")
        .body(s)
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    env::set_var("RUST_BACKTRACE", "0");
    env_logger::init();

    let sys = actix::System::new("ultimate");
  
    let _addr = server::new(|| {
        App::with_state(AppState {
            template: compile_templates!("./src/templates/**/*"),
        }).middleware(middleware::Logger::default())
            .resource("/", |r| r.f(index))
            .resource("/{test}/", |r| r.f(index))
            .handler(
                "/static",
                fs::StaticFiles::new("./src/static/build").show_files_listing(),
            )
            .default_resource(|r| {
                r.method(Method::GET).f(p404);
                r.route()
                    .filter(pred::Not(pred::Get()))
                    .f(|_req| HttpResponse::MethodNotAllowed());
                r.h(NormalizePath::default());
            })
    }).bind("127.0.0.1:8000")
        .expect("Can not bind to 127.0.0.1:8000")
        .start();

    let _ = sys.run();
}
