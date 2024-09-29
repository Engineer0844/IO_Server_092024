// Server that displays IO Status
use axum::{extract::State, response::Html};

use tokio::sync::mpsc::Sender;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use rppal::gpio::Gpio;
use rppal::i2c::I2c;
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::error::Error;
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod rhino;
mod web;
use web::app;

// ADS1115 I2C address when ADDR pin pulled to ground
const ADDR_ADS115: u16 = 0x48; // Address of first ADS115 chip
const ADDR_ADS115_TWO: u16 = 0x49; // Address of second ADS115 chip

// ADS115 register addresses.
const REG_CONFIGURATION: u8 = 0x01;
const REG_CONVERSION: u8 = 0x00;
const DELAY_TIME: u64 = 100;
const MAIN_LOOP_DELAY: u64 = 100;
const I2C_DELAY_TIME: u64 = 10;
const VOLTAGE_LIMIT: f32 = 6.5;

//Output setup

const OUTPUT20: u8 = 20;

enum OutputCommand {
    LedToggle(i32),
}

#[derive(Clone)]
struct IoState {
    // digital input pins.
    pin_one: bool,
    pin_two: bool,
    pin_three: bool,

    pub adc1_channel0: f32,
    pub adc1_channel1: f32,
    pub adc1_channel2: f32,
    pub adc1_channel3: f32,

    pub adc2_channel0: f32,
    pub adc2_channel1: f32,
    pub adc2_channel2: f32,
    pub adc2_channel3: f32,

    sneaky_sender: Sender<OutputCommand>,
}

impl IoState {
    fn new(tx: Sender<OutputCommand>) -> Self {
        Self {
            pin_one: false,
            pin_two: false,
            pin_three: false,

            adc1_channel0: 0.0,
            adc1_channel1: 0.0,
            adc1_channel2: 0.0,
            adc1_channel3: 0.0,
            adc2_channel0: 0.0,
            adc2_channel1: 0.0,
            adc2_channel2: 0.0,
            adc2_channel3: 0.0,
            
            sneaky_sender: tx,
        }
    }

    fn set_pin(&mut self, pin: u8, value: bool) {
        if pin == 0 {
            self.pin_one = value;
        } else if pin == 1 {
            self.pin_two = value;
        }
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    let io_state = IoState::new(tx);

    let shared_state: Arc<Mutex<IoState>> = Arc::new(Mutex::new(io_state));
   
    let background_state = shared_state.clone();

    // #[cfg(target_arch = "arm")]
    {
        let mut pin_pic_one = String::new();
        println!("Pick the first Raspberry PI output pin number"); //PIN 20 is connected for pin pick
        io::stdin()
            .read_line(&mut pin_pic_one)
            .expect("Failed to read line");
        let pick_one = get_pin_number(pin_pic_one);
        println!("pick_one equals: {pick_one}");

        let mut i2c = I2c::new()?;
        i2c.set_slave_address(ADDR_ADS115)?; // Set the I2C slave address to the device we're communicating with.

        i2c.block_write(REG_CONFIGURATION, &[0x42, 0x82])?; // Set configuration setting to ADS115
        tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME));

        i2c.block_write(REG_CONVERSION, &[0x00])?; // Set ADS115 config to look at the conversion registers
        tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME));
        let mut reg = [0u8; 2];

        i2c.block_read(0x00, &mut reg)?; // reads ADS115 conversion register and puts contents into reg buffer
        tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME));

        //let adc0val:u16 = u16::from_be_bytes(reg);
        //println!(" ADC 0 decimal value = {:?} ", adc0val);
        //let adc0voltage:f32 = adc0val.into();

        //let adc0voltage:f32 = adc0voltage * 0.000125;
        //println!(" ADC 0 voltage = {:?} ", adc0voltage);

        tokio::spawn(async move {
            // set input pins
            let pin_25 = Gpio::new().unwrap().get(25).unwrap().into_input_pulldown();
            let pin_24 = Gpio::new().unwrap().get(24).unwrap().into_input_pulldown();

            // set the user selected outputs
            let mut pin_selection_one = Gpio::new().unwrap().get(pick_one).unwrap().into_output();
            let mut output_20 = Gpio::new().unwrap().get(OUTPUT20).unwrap().into_output();
            output_20.set_low();
            loop {
                let adc1_channel0 = get_adc_value(ADDR_ADS115, 0x42, 0x82, "ADC1_CH0_Voltage")
                    .await
                    .unwrap();
                let adc1_channel1 = get_adc_value(ADDR_ADS115, 0x52, 0x82, "ADC1_CH1_Voltage")
                    .await
                    .unwrap();
                let adc1_channel2 = get_adc_value(ADDR_ADS115, 0x62, 0x82, "ADC1_CH2_Voltage")
                    .await
                    .unwrap();
                let adc1_channel3 = get_adc_value(ADDR_ADS115, 0x72, 0x82, "ADC1_CH3_Voltage")
                    .await
                    .unwrap();
                println!("");
                println!("");
                let adc2_channel0 = get_adc_value(ADDR_ADS115_TWO, 0x42, 0x82, "ADC2_CH0_Voltage")
                    .await
                    .unwrap();
                let adc2_channel1 = get_adc_value(ADDR_ADS115_TWO, 0x52, 0x82, "ADC2_CH1_Voltage")
                    .await
                    .unwrap();
                let adc2_channel2 = get_adc_value(ADDR_ADS115_TWO, 0x62, 0x82, "ADC2_CH2_Voltage")
                    .await
                    .unwrap();
                let adc2_channel3 = get_adc_value(ADDR_ADS115_TWO, 0x72, 0x82, "ADC2_CH3_Voltage")
                    .await
                    .unwrap();
                println!("");

                {
                    let mut io_state = background_state.lock().unwrap();
                    io_state.adc1_channel0 = adc1_channel0;
                    io_state.adc1_channel1 = adc1_channel1;
                    io_state.adc1_channel2 = adc1_channel2;
                    io_state.adc1_channel3 = adc1_channel3;

                    io_state.adc2_channel0 = adc2_channel0;
                    io_state.adc2_channel1 = adc2_channel1;
                    io_state.adc2_channel2 = adc2_channel2;
                    io_state.adc2_channel3 = adc2_channel3;

                    io_state.set_pin(0, pin_24.is_high());
                    io_state.set_pin(1, pin_25.is_high());
                }
                pin_selection_one.toggle();
                match rx.try_recv() {
                    Ok(OutputCommand::LedToggle(pin_id)) => {
                        println!("Got a message in main from rx: {}", pin_id);
                        if pin_id == 20 {
                            if output_20.is_set_low() {
                                output_20.set_high();
                            } else 
                            {
                                output_20.set_low();
                            }
                            
                        }
                    }
                    Err(_) => {} 
                }
                tokio::time::sleep(Duration::from_millis(MAIN_LOOP_DELAY)).await;
            }

            // sleep for half a second
            // toggle pin values
            // sleep a bit
            // toggle
        });
    }
    app(shared_state).await;

    Ok(())
}

// chip address u16; config reg1 u8, congfig reg2 u8;

fn get_pin_number(x: String) -> u8 {
    let num = x.trim().parse::<u8>().unwrap();
    println!("num equals: {num}");

    num
}

async fn get_adc_value(
    adc_address: u16,
    config_reg1: u8,
    config_reg2: u8,
    print_text: &str,
) -> Result<f32, Box<dyn Error>> {
    let mut adc_reg = [0u8; 2];

    let mut i2c0 = I2c::new()?;
    i2c0.set_slave_address(adc_address)?;

    i2c0.block_write(REG_CONFIGURATION, &[config_reg1, config_reg2])?; // Set configuration setting to ADS115
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_write(REG_CONVERSION, &[0x00])?; // Set ADS115 config to look at the conversion registers
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_read(REG_CONVERSION, &mut adc_reg)?; // reads ADS115 conversion register and puts contents into reg buffer
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    let adc_val: u16 = u16::from_be_bytes(adc_reg);
    //println!(" ADC 0 decimal value = {:?} ", adc0val);
    let adcvoltage: f32 = adc_val.into();

    let mut adcvoltage: f32 = adcvoltage * 0.000125;
    if adcvoltage > VOLTAGE_LIMIT {
        adcvoltage = 0.01;
    }
    println!("{} = {:.2?}", print_text, adcvoltage);

    //let adcvoltage = Decimal::new(adcvoltage, 2);

    Ok(adcvoltage)
}

async fn get_adc0_value() -> Result<f32, Box<dyn Error>> {
    let mut adc0_reg = [0u8; 2];

    let mut i2c0 = I2c::new()?;
    i2c0.set_slave_address(ADDR_ADS115)?;

    i2c0.block_write(REG_CONFIGURATION, &[0x42, 0x82])?; // Set configuration setting to ADS115
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_write(REG_CONVERSION, &[0x00])?; // Set ADS115 config to look at the conversion registers
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_read(REG_CONVERSION, &mut adc0_reg)?; // reads ADS115 conversion register and puts contents into reg buffer
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    let adc0val: u16 = u16::from_be_bytes(adc0_reg);
    //println!(" ADC 0 decimal value = {:?} ", adc0val);
    let adc0voltage: f32 = adc0val.into();

    let mut adc0voltage: f32 = adc0voltage * 0.000125;
    if adc0voltage > VOLTAGE_LIMIT {
        adc0voltage = 0.01;
    }
    println!(" ADC_1 0 voltage = {:?} ", adc0voltage);

    Ok(adc0voltage)
}

async fn get_adc1_value() -> Result<f32, Box<dyn Error>> {
    let mut adc1_reg = [0u8; 2];

    let mut i2c1 = I2c::new()?;
    i2c1.set_slave_address(ADDR_ADS115)?;

    i2c1.block_write(REG_CONFIGURATION, &[0x52, 0x82])?; // Set configuration setting to ADS115
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c1.block_write(REG_CONVERSION, &[0x00])?; // Set ADS115 config to look at the conversion registers
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c1.block_read(REG_CONVERSION, &mut adc1_reg)?; // reads ADS115 conversion register and puts contents into reg buffer
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    let adc1val: u16 = u16::from_be_bytes(adc1_reg);
    //println!(" ADC 1 decimal value = {:?} ", adc1val);
    let adc1voltage: f32 = adc1val.into();

    let mut adc1voltage: f32 = adc1voltage * 0.000125;
    if adc1voltage > VOLTAGE_LIMIT {
        adc1voltage = 0.01;
    }
    println!(" ADC_1 1 voltage = {:?} ", adc1voltage);

    Ok(adc1voltage)
}

async fn get_adc2_value() -> Result<f32, Box<dyn Error>> {
    let mut adc2_reg = [0u8; 2];

    let mut i2c2 = I2c::new()?;
    i2c2.set_slave_address(ADDR_ADS115)?;

    i2c2.block_write(REG_CONFIGURATION, &[0x62, 0x82])?; // Set configuration setting to ADS115
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c2.block_write(REG_CONVERSION, &[0x00])?; // Set ADS115 config to look at the conversion registers
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c2.block_read(REG_CONVERSION, &mut adc2_reg)?; // reads ADS115 conversion register and puts contents into reg buffer
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    let adc2val: u16 = u16::from_be_bytes(adc2_reg);
    //println!(" ADC 2 decimal value = {:?} ", adc2val);
    let adc2voltage: f32 = adc2val.into();

    let mut adc2voltage: f32 = adc2voltage * 0.000125;
    if adc2voltage > VOLTAGE_LIMIT {
        adc2voltage = 0.01;
    }
    println!(" ADC_1 2 voltage = {:?} ", adc2voltage);

    Ok(adc2voltage)
}

async fn get_adc3_value() -> Result<f32, Box<dyn Error>> {
    let mut adc3_reg = [0u8; 2];

    let mut i2c3 = I2c::new()?;
    i2c3.set_slave_address(ADDR_ADS115)?;

    i2c3.block_write(REG_CONFIGURATION, &[0x72, 0x82])?; // Set configuration setting to ADS115
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c3.block_write(REG_CONVERSION, &[0x00])?; // Set ADS115 config to look at the conversion registers
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c3.block_read(REG_CONVERSION, &mut adc3_reg)?; // reads ADS115 conversion register and puts contents into reg buffer
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    let adc3val: u16 = u16::from_be_bytes(adc3_reg);
    //println!(" ADC 3 decimal value = {:?} ", adc3val);
    let adc3voltage: f32 = adc3val.into();

    let mut adc3voltage: f32 = adc3voltage * 0.000125;
    if adc3voltage > VOLTAGE_LIMIT {
        adc3voltage = 0.01;
    }
    println!(" ADC_1 3 voltage = {:?} ", adc3voltage);

    Ok(adc3voltage)
}

async fn get_adc0_2_value() -> Result<f32, Box<dyn Error>> // this is a second ADS1115 ADC slave chip
{
    let mut adc0_2_reg = [0u8; 2];

    let mut i2c0 = I2c::new()?;
    i2c0.set_slave_address(ADDR_ADS115_TWO)?;

    i2c0.block_write(REG_CONFIGURATION, &[0x42, 0x82])?; // Set configuration setting to ADS115
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_write(REG_CONVERSION, &[0x00])?; // Set ADS115 config to look at the conversion registers
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_read(REG_CONVERSION, &mut adc0_2_reg)?; // reads ADS115 conversion register and puts contents into reg buffer
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    let adc0_2_val: u16 = u16::from_be_bytes(adc0_2_reg);
    //println!(" ADC 0 decimal value = {:?} ", adc0val);
    let adc0_2_voltage: f32 = adc0_2_val.into();

    let mut adc0_2_voltage: f32 = adc0_2_voltage * 0.000125;
    if adc0_2_voltage > VOLTAGE_LIMIT {
        adc0_2_voltage = 0.01;
    }
    println!(" ADC_2 0 voltage = {:?} ", adc0_2_voltage);

    Ok(adc0_2_voltage)
}

async fn get_adc1_2_value() -> Result<f32, Box<dyn Error>> // this is a second ADS1115 ADC slave chip
{
    let mut adc1_2_reg = [0u8; 2];

    let mut i2c0 = I2c::new()?;
    i2c0.set_slave_address(ADDR_ADS115_TWO)?;

    i2c0.block_write(REG_CONFIGURATION, &[0x52, 0x82])?; // Set configuration setting to ADS115
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_write(REG_CONVERSION, &[0x00])?; // Set ADS115 config to look at the conversion registers
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_read(REG_CONVERSION, &mut adc1_2_reg)?; // reads ADS115 conversion register and puts contents into reg buffer
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    let mut adc1_2_val: u16 = u16::from_be_bytes(adc1_2_reg);
    //println!(" ADC 0 decimal value = {:?} ", adc0val);
    let adc1_2_voltage: f32 = adc1_2_val.into();

    let mut adc1_2_voltage: f32 = adc1_2_voltage * 0.000125;
    if adc1_2_voltage > VOLTAGE_LIMIT {
        adc1_2_voltage = 0.01;
    }
    println!(" ADC_2 0 voltage = {:?} ", adc1_2_voltage);
    Ok(adc1_2_voltage)
}

async fn get_adc2_2_value() -> Result<f32, Box<dyn Error>> // this is a second ADS1115 ADC slave chip
{
    let mut adc2_2_reg = [0u8; 2];

    let mut i2c0 = I2c::new()?;
    i2c0.set_slave_address(ADDR_ADS115_TWO)?;

    i2c0.block_write(REG_CONFIGURATION, &[0x62, 0x82])?; // Set configuration setting to ADS115
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_write(REG_CONVERSION, &[0x00])?; // Set ADS115 config to look at the conversion registers
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_read(REG_CONVERSION, &mut adc2_2_reg)?; // reads ADS115 conversion register and puts contents into reg buffer
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    let adc2_2_val: u16 = u16::from_be_bytes(adc2_2_reg);
    //println!(" ADC 0 decimal value = {:?} ", adc0val);
    let adc2_2_voltage: f32 = adc2_2_val.into();

    let mut adc2_2_voltage: f32 = adc2_2_voltage * 0.000125;
    if adc2_2_voltage > VOLTAGE_LIMIT {
        adc2_2_voltage = 0.01;
    }
    println!(" ADC_2 2 voltage = {:?} ", adc2_2_voltage);

    Ok(adc2_2_voltage)
}

async fn get_adc3_2_value() -> Result<f32, Box<dyn Error>> // this is a second ADS1115 ADC slave chip
{
    let mut adc3_2_reg = [0u8; 2];

    let mut i2c0 = I2c::new()?;
    i2c0.set_slave_address(ADDR_ADS115_TWO)?;

    i2c0.block_write(REG_CONFIGURATION, &[0x72, 0x82])?; // Set configuration setting to ADS115
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_write(REG_CONVERSION, &[0x00])?; // Set ADS115 config to look at the conversion registers
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    i2c0.block_read(REG_CONVERSION, &mut adc3_2_reg)?; // reads ADS115 conversion register and puts contents into reg buffer
    tokio::time::sleep(Duration::from_millis(I2C_DELAY_TIME)).await;

    let adc3_2_val: u16 = u16::from_be_bytes(adc3_2_reg);
    //println!(" ADC 0 decimal value = {:?} ", adc0val);
    let adc3_2_voltage: f32 = adc3_2_val.into();

    let mut adc3_2_voltage: f32 = adc3_2_voltage * 0.000125;
    if adc3_2_voltage > VOLTAGE_LIMIT {
        adc3_2_voltage = 0.01;
    }
    println!(" ADC_2 3 voltage = {:?} ", adc3_2_voltage);

    Ok(adc3_2_voltage)
}
