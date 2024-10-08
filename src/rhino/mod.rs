/// Provides management of communicating between the back end and the front end.
use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures::stream::SplitSink;
use futures::stream::SplitStream;
use futures::SinkExt;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

use crate::OutputCommand;

pub struct TextDisplay {}

/// Send from the server to the client.
#[derive(Serialize, Deserialize)]
pub struct TextUpdate {
    id: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LEDToggleUpdate {
    id: String,
    message: String,
}
pub enum ServerCommands {}

pub enum ClientCommands {}

pub struct InnerRhino {}

// has an maintance loop for shuttling data back and forth.
pub struct Rhino {
    sender: SplitSink<WebSocket, Message>,
}

struct RhinoMaintainer {}

impl RhinoMaintainer {
    async fn maintenance(mut receiver: SplitStream<WebSocket>, output_sender: Sender<OutputCommand>) {
        // read commands from the client.
        loop {
            let msg = receiver.next().await;
            match msg {
                Some(Ok(Message::Text(msg))) => {
                    let led_toggle: LEDToggleUpdate = serde_json::from_str(&msg).unwrap();
                    println!("Got a message from a client: {:?}", led_toggle);
                    output_sender.send(OutputCommand::LedToggle(led_toggle.id.parse().unwrap())).await;
                }
                Some(_) => { 
                    println!("Iunno got somethign weird");
                }
                None => {
                    println!("the stream was closed");
                    break;
                }
            }
        }
    }
}

impl Rhino {
    pub fn new(socket: WebSocket, output_sender: Sender<OutputCommand>) -> Self {
        let (sender, receiver) = socket.split();

        tokio::spawn(RhinoMaintainer::maintenance(receiver, output_sender));
        Self { sender }
    }

    pub async fn send_text_update(&mut self, id: &str, text: String) {
        // Set the value of "some widget or something that is displaying aDC info"
        // self.state.get_widget(id).set_value(value);
        let text_update = TextUpdate {
            id: id.into(),
            text,
        };
        self.sender
            .send(Message::Text(serde_json::to_string(&text_update).unwrap()))
            .await;
    }
}
