use std::error::Error;

use custom_sacn_controller::{PixelStrip};
use ddp_rs::{connection, protocol};

const LED_COUNT: usize = 450; //There are 450 LEDs. This was confirmed.

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting.");

    let mut pixels = PixelStrip::create(LED_COUNT);

    let mut conn = connection::DDPConnection::try_new(
        "10.10.30.17:4048",               // The IP address of the device followed by :4048
        protocol::PixelConfig::default(), // Default is RGB, 8 bits ber channel
        protocol::ID::Default,
        std::net::UdpSocket::bind("0.0.0.0:6969").unwrap(), // can be any unused port on 0.0.0.0, but protocol recommends 4048
    )?;

    std::thread::sleep(std::time::Duration::from_millis(2000));
    // this crate is non blocking, so with out the sleep, it will send them all instantly

    for _ in 0..200
    {
        for i in 0..LED_COUNT {
            for a in 0..i {
                pixels.set_pixel(a, Some(u8::MAX), Some(0), Some(0));
            }
            for a in i..LED_COUNT {
                pixels.set_pixel(a, Some(0), Some(0), Some(u8::MAX));
            }

            pixels.write_to_connection(&mut conn)?;

            std::thread::sleep(std::time::Duration::from_millis(10));
            // this crate is non blocking, so with out the sleep, it will send them all instantly
        }

        for i in 0..LED_COUNT {
            for a in 0..i {
                pixels.set_pixel(a,  Some(0), Some(u8::MAX),Some(0));
            }
            for a in i..LED_COUNT {
                pixels.set_pixel(a, Some(u8::MAX), Some(0), Some(0));
            }

            pixels.write_to_connection(&mut conn)?;

            std::thread::sleep(std::time::Duration::from_millis(10));
            // this crate is non blocking, so with out the sleep, it will send them all instantly
        }

    }

    println!("Finished.");
    Ok(())
}
