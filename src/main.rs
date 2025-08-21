use std::{error::Error, net::SocketAddr};

use angular_units::Deg;
use custom_ddp_controller::{
    displays::demos,
    pixels::{pixel::HSV, pixelstrip::PixelStrip},
    services::LedCommandHandler,
};
use ddp_rs::{
    connection::{self, DDPConnection},
    protocol,
};

const LED_COUNT: usize = 450; //There are 450 LEDs. This was confirmed.

use std::{
    env,
    net::{IpAddr, Ipv4Addr},
};

use hyper_services::{service::stateful_service::StatefulService, spawn_server};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let port = match args.get(1) {
        Some(port) => match port.parse::<u16>() {
            Ok(port) => port,
            Err(e) => {
                println!("Invalid port {}", port);
                println!("{}", e.to_string());
                return;
            }
        },
        None => {
            println!("Provide the outbound port as the first argument.");
            return;
        }
    };

    let pixels = PixelStrip::create(LED_COUNT);

    let outbound_port = std::net::UdpSocket::bind("0.0.0.0:6969"); // can be any unused port on 0.0.0.0, but protocol recommends 4048

    let outbound_port = match outbound_port {
        Ok(port) => port,
        Err(e) => {
            eprintln!("Couldn't bind port.");
            eprintln!("{:?}", e);
            return;
        }
    };

    let socket_address: SocketAddr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 10, 30, 17)), 8080);

    let conn = DDPConnection::try_new(
        socket_address,                   // The IP address of the device followed by :4048
        protocol::PixelConfig::default(), // Default is RGB, 8 bits ber channel
        protocol::ID::Default,
        outbound_port,
    );

    let conn = match conn {
        Ok(conn) => conn,
        Err(e) => {
            eprintln!("Couldn't create DDP Connection.");
            eprintln!("{:?}", e);
            return;
        }
    };

    //demos::red_green_blue(conn, pixels)?;
    //demos::hue_progression(conn, pixels)?;
    demos::rainbow_oscillation(conn, pixels).unwrap();

    println!("Starting REST Service");

    let handler = LedCommandHandler::new();

    let event_server = spawn_server(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        port,
        StatefulService::create(handler),
    );

    match event_server.await {
        Ok(_) => println!("Closed REST Service Gracefully"),
        Err(e) => {
            println!("REST Service Failure");
            println!("{}", e.to_string());
        }
    };
}
