//! The server binary: `PORT`, `MEAL_LOG_CSV_URL` and the submission env vars
//! behave as they do in the Scala.

use std::net::TcpListener;

fn main() {
    let env = |key: &str| std::env::var(key).ok();
    let context = std::sync::Arc::new(recibase_server::Context::new(Box::new(env)));
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8081);
    let listener = TcpListener::bind(("0.0.0.0", port)).expect("bind");
    eprintln!("Recibase listening on 0.0.0.0:{}", port);
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        // Nagle's algorithm plus delayed ACKs would hold back the second
        // segment of a multi-segment response; every real HTTP stack (http4s on
        // Netty/Ember included) turns it off.
        let _ = stream.set_nodelay(true);
        let context = std::sync::Arc::clone(&context);
        std::thread::spawn(move || {
            let _ = serve(stream, &context);
        });
    }
}

fn serve(
    mut stream: std::net::TcpStream,
    context: &recibase_server::Context,
) -> std::io::Result<()> {
    loop {
        let Some(request) = recibase_server::http::read_request(&stream)? else {
            return Ok(());
        };
        let keeps_alive = request.keeps_alive();
        let response = recibase_server::route(&request, context);
        let date = recibase_server::http::http_date(chrono::Utc::now());
        let connection = if keeps_alive { "keep-alive" } else { "close" };
        response.write_to(&mut stream, connection, &date)?;
        if !keeps_alive {
            return Ok(());
        }
    }
}
