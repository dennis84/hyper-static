extern crate futures;
extern crate hyper;
extern crate flate2;

use std::fs::File;
use std::ffi::OsStr;
use std::path::Path;
use std::io::{Read, Write};

use futures::future;
use futures::future::FutureResult;

use hyper::{Request, Response, StatusCode, Body};

use flate2::Compression;
use flate2::write::GzEncoder;

fn get_content_type(path: &Path) -> &str {
    match path.extension().and_then(OsStr::to_str) {
        Some("html") => "text/html",
        Some("css")  => "text/css",
        Some("js")   => "text/javascript",
        _            => "text/plain",
    }
}

pub fn from_dir(base_path: &str, req: Request<Body>)
        -> FutureResult<Response<Body>, hyper::Error> {
    let file_path = format!("{}{}", base_path, req.uri().path());
    let file_path = Path::new(&file_path);
    if !file_path.is_file() {
        return future::ok(Response::new()
            .with_status(StatusCode::NotFound)
            .with_body("Not Found"));
    }

    let mut file = File::open(file_path).unwrap();
    let mut body = Vec::new();
    file.read_to_end(&mut body).unwrap();

    let mut encoder = GzEncoder::new(Vec::new(), Compression::Best);
    encoder.write_all(body.as_slice()).unwrap();
    let compressed_bytes = encoder.finish().unwrap();

    let mut response = Response::new();
    response.headers_mut().set_raw("Content-Type", get_content_type(file_path));
    response.headers_mut().set_raw("Content-Encoding", "gzip");
    response.set_body(compressed_bytes);
    future::ok(response)
}
