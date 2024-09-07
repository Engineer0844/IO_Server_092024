use axum::extract::ws::WebSocket;
use serde::{Serialize, Deserialize};
/// Provides management of communicating between the back end and the front end. 
use axum::extract::ws::Message;

pub struct TextDisplay {

}

/// Send from the server to the client.
#[derive(Serialize, Deserialize)]
pub struct TextUpdate {
    id: String,
    text: String,
}


pub enum ServerCommands { 

}

pub enum ClientCommands {

}

pub struct InnerRhino {

}

// has an maintance loop for shuttling data back and forth.
pub struct Rhino { 
    socket: WebSocket,
}

impl Rhino { 

    pub fn new(socket: WebSocket)  -> Self {
        Self {
            socket
        }
    }

    pub async fn send_text_update(&mut self, id: &str, text: String) {
        // Set the value of "some widget or something that is displaying aDC info"
        // self.state.get_widget(id).set_value(value);
        let text_update = TextUpdate {id: id.into(), text};
        self.socket.send(Message::Text(serde_json::to_string(&text_update).unwrap())).await;
    }

    pub fn maintance(&self) {
        
        // read commands from the client.
        loop { 

        }
    }
}
