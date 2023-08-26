use anyhow::{Context, Result};
use wasi_http_tests::bindings::wasi::http::types::{Method, Scheme};

struct Component;

fn main() {}

async fn run() -> Result<(), ()> {
    let res = wasi_http_tests::request(
        Method::Post,
        Scheme::Http,
        "localhost:3000",
        "/post",
        Some(b"{\"foo\": \"bar\"}"),
        None,
    )
    .await
    .context("localhost:3000 /post")
    .unwrap();

    println!("localhost:3000 /post: {res:?}");
    assert_eq!(res.status, 200);
    let method = res.header("x-wasmtime-test-method").unwrap();
    assert_eq!(std::str::from_utf8(method).unwrap(), "POST");
    assert_eq!(res.body, b"{\"foo\": \"bar\"}");

    Ok(())
}

impl wasi_http_tests::bindings::exports::wasi::cli::run::Run for Component {
    fn run() -> Result<(), ()> {
        wasi_http_tests::in_tokio(async { run().await })
    }
}

wasi_http_tests::export_command_extended!(Component);
