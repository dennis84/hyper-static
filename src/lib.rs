extern crate mime;
extern crate futures;
extern crate hyper;
extern crate flate2;

use std::fs::File;
use std::ffi::OsStr;
use std::path::Path;
use std::io::{Read, Write};

use futures::future;
use futures::future::FutureResult;

use hyper::server::{Request, Response};
use hyper::header;
use hyper::StatusCode;

use flate2::Compression;
use flate2::write::GzEncoder;

fn get_content_type(path: &Path) -> header::ContentType {
    match path.extension().and_then(OsStr::to_str) {
        Some("html") => header::ContentType(mime::TEXT_HTML),
        Some("css")  => header::ContentType(mime::TEXT_CSS),
        Some("js")   => header::ContentType(mime::TEXT_JAVASCRIPT),
        _            => header::ContentType(mime::TEXT_PLAIN),
    }
}

pub fn from_dir(base_path: &str, req: Request)
        -> FutureResult<Response, hyper::Error> {
    let file_path = format!("{}{}", base_path, req.path());
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

    future::ok(Response::new()
        .with_header(get_content_type(file_path))
        .with_header(header::ContentEncoding(vec![
            header::Encoding::Gzip
        ]))
        .with_body(compressed_bytes))
}
