extern crate mdns;

use mdns::{Record, RecordKind};
use std::net::IpAddr;

/// The hostname of the devices we are searching for.
/// Every Chromecast will respond to the service name in this example.
const SERVICE_NAME: &'static str = "_googlecast._tcp.local";

pub fn run() -> Result<(), mdns::Error> {
    // Iterate through responses from each cast device
    let responses = mdns::discover::all(SERVICE_NAME)?.timeout(std::time::Duration::new(5, 0));

    for response in responses {
        match response?.records().filter_map(to_ip_addr).next() {
            None => println!("cast device does not advertise address"),
            Some(addr) => println!("Found cast device at {}", addr),
        }
    }

    Ok(())
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}
