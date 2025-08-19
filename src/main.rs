use std::error::Error;

use angular_units::Deg;
use custom_ddp_controller::{
    displays::demos,
    pixels::{pixel::HSV, pixelstrip::PixelStrip},
};
use ddp_rs::{
    connection::{self, DDPConnection},
    protocol,
};

const LED_COUNT: usize = 450; //There are 450 LEDs. This was confirmed.

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting.");

    let pixels = PixelStrip::create(LED_COUNT);

    let conn: DDPConnection = DDPConnection::try_new(
        "10.10.30.17:4048",               // The IP address of the device followed by :4048
        protocol::PixelConfig::default(), // Default is RGB, 8 bits ber channel
        protocol::ID::Default,
        std::net::UdpSocket::bind("0.0.0.0:6969").unwrap(), // can be any unused port on 0.0.0.0, but protocol recommends 4048
    )?;

    //demos::red_green_blue(conn, pixels)?;
    //demos::hue_progression(conn, pixels)?;
    demos::rainbow_oscillation(conn, pixels)?;

    println!("Finished.");

    Ok(())
}
