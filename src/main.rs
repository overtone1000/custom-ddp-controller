use std::{error::Error, net::{IpAddr, SocketAddr}, thread::sleep, time::Duration};

use sacn::{packet::ACN_SDT_MULTICAST_PORT, source::SacnSource};

fn main() -> Result<(), Box<dyn Error>> {    

    let local_addr: SocketAddr = SocketAddr::new(IpAddr::V4("0.0.0.0".parse().unwrap()), ACN_SDT_MULTICAST_PORT + 1);

    let mut src = SacnSource::with_ip("Source", local_addr).unwrap();

    let universe: u16 = 1;
    let sync_uni: Option<u16> = Some(1); // Data packets use a synchronisation address of 1.
    let priority: Option<u8> = Some(100); // The priority for the sending data, must be 1-200 inclusive,  None means use default.
    
    // To send using unicast the dst_ip argument is set to a Some() value with the address to send the data to. By default the port should be the
    // ACN_SDT_MULTICAST_PORT but this can be configured differently if required in a specific situation. Change this address to the correct address for your
    // application, 192.168.0.1 is just a stand-in.
    let destination_address: SocketAddr = SocketAddr::new(IpAddr::V4("10.10.30.17".parse().unwrap()), ACN_SDT_MULTICAST_PORT);
    let dst_ip: Option<SocketAddr> = Some(destination_address);

    src.register_universe(universe).unwrap(); // Register with the source that will be sending on the given universe.

    let mut data: Vec<u8> = vec![0, 0, 0, 0, 255, 255, 128, 128]; // Some arbitrary data, must have length <= 513 (including start-code).

    // Actually send the data, since the sync_uni is not 0 the data will be synchronised at the receiver (if the receiver supports synchronisation).
    src.send(&[universe], &data, priority, dst_ip, sync_uni).unwrap();

    // A small delay between sending data and sending the sync packet as recommend in ANSI E1.31-2018 Section 11.2.2.
    sleep(Duration::from_millis(10));
    
    // To actually trigger the data need to send a synchronisation packet like so.
    src.send_sync_packet(sync_uni.unwrap(), dst_ip).unwrap();
    
    Ok(())
}