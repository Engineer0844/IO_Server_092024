/// https://momori.dev/posts/building-a-websocket-chat-app-with-axum-and-react/
/// Website related functionality goes here.
// pulled from https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs
use axum::{
    extract::ws::{WebSocket, WebSocketUpgrade},
    http::{StatusCode, Uri},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower_http::services::{ServeDir, ServeFile};

use crate::IoState;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::rhino::Rhino;

/// Main application that launches the server
///
pub async fn app(shared_state: Arc<Mutex<IoState>>) {
    println!("Launching web server");

    let serve_dir = ServeDir::new("assets");

    let app = Router::new()
        .route("/index", get(index))
        .route("/ws", get(ws_handler))
        .route("/io", get(crate::get_io_status))
        .nest_service("/", serve_dir.clone())
        .fallback_service(serve_dir.clone())
        .fallback(fallback)
        .with_state(shared_state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    log::debug!("custom Fallback service: {uri}");
    (StatusCode::NOT_FOUND, format!("no route for {uri}"))
}

async fn index() -> (StatusCode, Html<String>) {
    println!("no such file assets/index.html");
    let string = std::fs::read_to_string("assets/index.html").unwrap();
    (StatusCode::OK, Html(string))
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    println!("Ws handler got called");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(socket: WebSocket) {
    // returning from the handler closes the websocket connection
    println!("Websocket context marques destroyed");

    let mut rhino = Rhino::new(socket);

    let mut counter = 0;

    loop {
        println!("counter: {counter}");

        rhino.send_text_update("counter", counter.to_string()).await;
        rhino.send_text_update("adc-1", counter.to_string()).await;
        rhino
            .send_text_update("adc-2", (counter * 2).to_string())
            .await;
        rhino
            .send_text_update("adc-3", (counter * 3).to_string())
            .await;

        // socket.send(Message::Text(counter.to_string())).await;
        tokio::time::sleep(Duration::from_millis(500)).await;

        counter += 1;
    }
}
