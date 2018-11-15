use std::env;

extern crate actix_web;
extern crate vmctl_controller;

use actix_web::{server, App, HttpRequest};
use std::str;
use vmctl_controller::status::Status;

fn index(_req: &HttpRequest) -> &'static str {
    "Hello World 2"
}

fn show(_req: &HttpRequest) -> String {
    let r = Status::new().unwrap();
    format!(
        "{} {} {} {} {} {} {} {}",
        r[0].id, r[0].pid, r[0].vcpus, r[0].max_mem, r[0].cur_mem, r[0].tty, r[0].owner, r[0].name
    )
}

fn start(req: &HttpRequest) -> String {
    let n = req.match_info().get("name").unwrap();
    if vmctl_controller::start(n).is_ok() {
        return "Started\n".to_string();
    }
    return "Not started\n".to_string();
}

fn stop(req: &HttpRequest) -> String {
    let n = req.match_info().get("name").unwrap();
    if vmctl_controller::stop(n).is_ok() {
        return "Stopped\n".to_string();
    }
    return "Not running\n".to_string();
}

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> String {
    env::var("PORT").ok().unwrap_or("8080".to_string())
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(index))
            .prefix("/vms")
            .resource("/show", |r| r.f(show))
            .resource("/start/{name}", |r| r.f(start))
            .resource("/stop/{name}", |r| r.f(stop))
    })
    .bind(format!("{}:{}", "0.0.0.0", get_server_port()))
    .unwrap()
    .run();
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestServer;
    use actix_web::HttpMessage;
    use std::str;
    #[test]
    fn index_test() {
        let mut srv = TestServer::new(|app| app.handler(index));

        let request = srv.get().finish().unwrap();
        let response = srv.execute(request.send()).unwrap();
        assert!(response.status().is_success());

        let bytes = srv.execute(response.body()).unwrap();
        let body = str::from_utf8(&bytes).unwrap();
        assert_eq!(body, "Hello World 2");
    }
}
