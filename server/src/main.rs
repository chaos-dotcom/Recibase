//! The server binary: `PORT`, `MEAL_LOG_CSV_URL` and the submission env vars
//! behave as they do in the Scala.

use std::net::TcpListener;

fn main() {
    let env = |key: &str| std::env::var(key).ok();
    let context = std::sync::Arc::new(recibase_server::Context::new(Box::new(env)));
    let port = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8081);
    let listener = TcpListener::bind(("0.0.0.0", port)).expect("bind");
    eprintln!("Recibase listening on 0.0.0.0:{}", port);
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let context = std::sync::Arc::clone(&context);
        std::thread::spawn(move || {
            let _ = serve(stream, &context);
        });
    }
}

fn serve(mut stream: std::net::TcpStream, context: &recibase_server::Context) -> std::io::Result<()> {
    loop {
        let Some(request) = recibase_server::http::read_request(&stream)? else {
            return Ok(());
        };
        let close = match request.header("connection") {
            Some(value) => value.eq_ignore_ascii_case("close"),
            None => false,
        };
        let response = recibase_server::route(&request, context);
        let date = recibase_server::http::http_date(chrono::Utc::now());
        response.write_to(&mut stream, close, &date)?;
        if close {
            return Ok(());
        }
    }
}
