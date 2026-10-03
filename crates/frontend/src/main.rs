
//! The frontend binary: `app.py` plus the server `gunicorn` provides.

use std::net::TcpListener;
use std::sync::Arc;

use recibase_frontend::app::App;
use recibase_frontend::http;
use recibase_frontend::version::resolve_deployed_version;

fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(8080);
    let backend_url = std::env::var("BACKEND_URL")
        .unwrap_or_else(|_| "http://localhost:8081/".to_string());
    let frontend_version = resolve_deployed_version();

    let app = Arc::new(App::new(backend_url, frontend_version, port));
    let listener = match TcpListener::bind(("0.0.0.0", port)) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("cannot listen on 0.0.0.0:{}: {}", port, error);
            std::process::exit(1);
        }
    };
    println!("Recibase frontend listening on 0.0.0.0:{}", port);
    println!("  backend : {}", app.backend_url);
    println!("  static  : {}", app.statics.root().display());
    if !app.statics.root().is_dir() {
        eprintln!(
            "warning: {} is not a directory, so /static/ will answer 404. Set STATIC_DIR \
             to the directory holding styles.css, or copy it beside the binary.",
            app.statics.root().display()
        );
    }
    println!("  version : {}", app.frontend_version);

    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let app = Arc::clone(&app);
        std::thread::spawn(move || serve(stream, app));
    }
}

fn serve(mut stream: std::net::TcpStream, app: Arc<App>) {
    loop {
        let request = match http::read_request(&stream) {
            Ok(Some(request)) => request,
            _ => return,
        };
        let keep_alive = request.keeps_alive();
        let response = app.handle(&request);
        if http::write_response(&mut stream, &response, &request).is_err() {
            return;
        }
        if !keep_alive {
            return;
        }
    }
}
