use pcap::{Device, Capture};
use anyhow::{Result, Context};

use crate::parsers;

pub struct Sniffer {
    device_name: String,
}

impl Sniffer {
    pub fn new(device_name: &str) -> Self {
        Self {
            device_name: device_name.to_string(),
        }
    }

    pub fn run(&self) -> Result<()> {
        println!("Configuring sniffer for device: {}", self.device_name);

        let device = Device::list()?
            .into_iter()
            .find(|d| d.name == self.device_name)
            .context(format!("Device '{}' not found", self.device_name))?;

        let mut cap = Capture::from_device(device)?
            .promisc(true)
            .snaplen(5000)
            .timeout(1000)
            .open()?;

        println!("Listening on {}...", self.device_name);

        while let Ok(packet) = cap.next_packet() {
            // Pass the data to the parser
            parsers::handle_packet(packet.data);
        }

        Ok(())
    }
}