/// Provides management of communicating between the back end and the front end. 


pub struct TextDisplay {

}


pub enum ServerCommands { 

}

pub enum ClientCommands {

}

pub struct InnerRhino {

}

// has an maintance loop for shuttling data back and forth.
#[derive(Clone)]
pub struct Rhino { 
        
}

impl Rhino { 

    pub fn set_pin_adc_value(&mut self, _id: usize, _value: f32) {
        // Set the value of "some widget or something that is displaying aDC info"
        // self.state.get_widget(id).set_value(value);
        
    }

    pub fn maintance(&self) {
        
        // read commands from the client.
        loop { 

        }
    }
}
