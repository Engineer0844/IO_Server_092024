// Server that displays IO Status

use axum::{
    extract::State,
    routing::get,
    Router,
    response::Html,
};


use std::sync::{Arc, Mutex};
use std::time::Duration;
use rppal::gpio::Gpio;
use std::io;
use std::any::type_name;
use std::error::Error;
use std::thread;
use rppal::pwm::{Channel, Polarity, Pwm};



async fn hello_world() -> String {
    let a = 40;
    format!("Hello, World! i am {}", a)
}

// const IO_PAGE: &str = ;

async fn get_io_status(State(state): State<Arc<Mutex<IoState>>>) -> Html<String> {
    let io_state = state.lock().unwrap();

    Html(format!(r#"
    <html>
    <title>Io Page</title>
    <body>
    Pin 24: {} Pin 25: {}
    <script>
    setTimeout(function() {{
        document.location.reload(true);
    }}, 50);
    </script>
    </body>
    "#, io_state.pin_one, io_state.pin_two))
}

struct IoState {
    // digital input pins.
    pin_one: bool,
    pin_two: bool,
    pin_three: bool, 
}

impl IoState {
    fn new() -> Self {
        Self {
            pin_one: false,
            pin_two: false,
            pin_three: false,
        }
    }

    fn set_pin(&mut self, pin: u8, value: bool) {
        if pin == 0{ 
            self.pin_one = value;
        }
        else if pin == 1 { 
            self.pin_two = value;
        }
    }
}

#[tokio::main]
async fn main() {

    let shared_state: Arc<Mutex<IoState>> = Arc::new(Mutex::new(IoState::new()));
    let background_state = shared_state.clone();

    let mut pin_pic_one = String::new();
    println!("Pick the first Raspberry PI output pin number"); //PIN 20 is connected for pin pick
    io::stdin().read_line(&mut pin_pic_one).expect("Failed to read line");

    let mut pick_one = get_pin_number(pin_pic_one);
    println!("pick_one equals: {pick_one}");


    tokio::spawn(async move {
        // set input pins
        let mut pin_25 = Gpio::new().unwrap().get(25).unwrap().into_input_pulldown();
        let mut pin_24 = Gpio::new().unwrap().get(24).unwrap().into_input_pulldown();
        // set the user selected outputs 
        let mut pin_selection_one = Gpio::new().unwrap().get(pick_one).unwrap().into_output();

    20

        

        loop {
            {
                let mut io_state = background_state.lock().unwrap();
                io_state.set_pin(0, pin_24.is_high());
                io_state.set_pin(1, pin_25.is_high());

                pin_selection_one.toggle();
                thread::sleep(Duration::from_millis(250));

            
            
            }
            tokio::time::sleep(Duration::from_millis(500)).await;
            // sleep for half a second
            // toggle pin values
            // sleep a bit 
            // toggle
        }
    });

    // build our application with a single route
    let app = Router::new()
                    .route("/", get(hello_world))
                    .route("/io", get(get_io_status))
                    .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn get_pin_number(x: String) -> u8 {

    let mut num = x.trim();
    let mut num = num.parse::<u8>().unwrap();
    println!("num equals: {num}");
    
    num   
}