
/// https://momori.dev/posts/building-a-websocket-chat-app-with-axum-and-react/
/// Website related functionality goes here. 

// pulled from https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/main.rs

use axum::{
    Router,
    routing::get,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    
};
use axum_extra::TypedHeader;

use std::net::SocketAddr;

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;

//allows to split the websocket stream into separate TX and RX branches
use futures::stream::StreamExt;

use std::sync::{Arc,Mutex};
use crate::IoState;

/// Main application that launches the server
///
pub async fn app(shared_state: Arc<Mutex<IoState>>) {
    let app = Router::new()
        .route("/index", get(index))
        .route("/ws", get(ws_handler))
        .route("/io", get(crate::get_io_status))
        .with_state(shared_state);
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


/// Returns out the index page. 
async fn index<'a>() -> Html<&'a str> { 
    Html(include_str!("../../static/index.html"))
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
async fn ws_handler(
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    println!("Ws handler got called");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket))
}

/// Actual websocket statemachine (one will be spawned per connection)
async fn handle_socket(mut socket: WebSocket) {
  
    // returning from the handler closes the websocket connection
    println!("Websocket context marques destroyed");
}
