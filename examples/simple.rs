extern crate futures;
extern crate hyper;
extern crate hyper_static;

use futures::future;

use hyper::server::{Http, Service, Request, Response};

struct Hello;

impl Service for Hello {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = future::FutureResult<Response, hyper::Error>;

    fn call(&self, req: Request) -> Self::Future {
        hyper_static::from_dir("public", req)
    }
}

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Hello)).unwrap();
    server.run().unwrap();
}
