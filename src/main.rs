use std::{
    error::Error,
    net::{IpAddr, SocketAddr},
    thread::sleep,
    time::Duration,
};

use ddp_rs::{connection, protocol};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting.");

    let mut conn = connection::DDPConnection::try_new(
        "10.10.30.17:4048",               // The IP address of the device followed by :4048
        protocol::PixelConfig::default(), // Default is RGB, 8 bits ber channel
        protocol::ID::Default,
        std::net::UdpSocket::bind("0.0.0.0:6969").unwrap(), // can be any unused port on 0.0.0.0, but protocol recommends 4048
    )?;

    // loop sets some colors for the first 6 pixels to see if it works
    for i in 0u8..100u8 {
        let high = 10u8.overflowing_mul(i).0;

        // loop through some colors

        let temp: usize = conn.write(&[
            high, /*red value*/
            0,    /*green value*/
            0,    /*blue value*/
            high, /*red value*/
            0,    /*green value*/
            0,    /*blue value*/
            0,    /*red value*/
            high, /*green value*/
            0,    /*blue value*/
            0,    /*red value*/
            high, /*green value*/
            0,    /*blue value*/
            0,    /*red value*/
            0,    /*green value*/
            high, /*blue value*/
            0,    /*red value*/
            0,    /*green value*/
            high, /*blue value*/
        ])?;

        std::thread::sleep(std::time::Duration::from_millis(10));
        // this crate is non blocking, so with out the sleep, it will send them all instantly

        println!("sent {temp} packets");
    }

    println!("Finished.");
    Ok(())
}
