use std::error::Error;

use custom_sacn_controller::{write_pixels, PixelRgb};
use ddp_rs::{connection, protocol};

const LED_COUNT: usize = 450; //There are 450 LEDs. This was confirmed.

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting.");

    let mut pixels: [PixelRgb; LED_COUNT] = [PixelRgb {
        red: 0,
        green: 0,
        blue: 0,
    }; LED_COUNT];

    let mut conn = connection::DDPConnection::try_new(
        "10.10.30.17:4048",               // The IP address of the device followed by :4048
        protocol::PixelConfig::default(), // Default is RGB, 8 bits ber channel
        protocol::ID::Default,
        std::net::UdpSocket::bind("0.0.0.0:6969").unwrap(), // can be any unused port on 0.0.0.0, but protocol recommends 4048
    )?;

    write_pixels(&mut conn, &pixels)?;

    std::thread::sleep(std::time::Duration::from_millis(2000));
    // this crate is non blocking, so with out the sleep, it will send them all instantly

    for i in 0..LED_COUNT {
        for a in 0..i {
            pixels[a].red = u8::MAX;
            pixels[a].green = 0;
            pixels[a].blue = 0;
        }
        for a in i..LED_COUNT {
            pixels[a].red = 0;
            pixels[a].green = 0;
            pixels[a].blue = u8::MAX;
        }

        write_pixels(&mut conn, &pixels)?;

        std::thread::sleep(std::time::Duration::from_millis(10));
        // this crate is non blocking, so with out the sleep, it will send them all instantly
    }

    for i in 0..LED_COUNT {
        for a in 0..i {
            pixels[a].red = 0;
            pixels[a].green = u8::MAX;
            pixels[a].blue = 0;
        }
        for a in i..LED_COUNT {
            pixels[a].red = u8::MAX;
            pixels[a].green = 0;
            pixels[a].blue = 0;
        }

        write_pixels(&mut conn, &pixels)?;

        std::thread::sleep(std::time::Duration::from_millis(10));
        // this crate is non blocking, so with out the sleep, it will send them all instantly
    }

    println!("Finished.");
    Ok(())
}
