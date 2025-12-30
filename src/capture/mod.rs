pub mod device_info;
pub mod device_info_list;

use anyhow::{Context, Result};
use pcap::{Capture, Device};
use std::sync::mpsc::Sender;

use crate::parsers;
use device_info::DeviceInfo;
use device_info_list::DeviceInfoList;

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

        let mut devices = DeviceInfoList::new();

        while let Ok(packet) = cap.next_packet() {
            if let Some(info) = parsers::handle_packet(packet.data) {
                // Append if not present: prefer MAC matching when available; otherwise by name
                let already_present = if let Some(mac) = info.mac_addr() {
                    devices.find_by_mac(mac).is_some()
                } else {
                    false
                };

                if !already_present {
                    // Print newly discovered device info
                    println!("New device discovered: {}", info);
                    devices.push(info);
                }
            }
        }

        Ok(())
    }

    pub fn run_with_channel(&self, tx: Sender<DeviceInfo>) -> Result<()> {
        let device = Device::list()?
            .into_iter()
            .find(|d| d.name == self.device_name)
            .context(format!("Device '{}' not found", self.device_name))?;

        let mut cap = Capture::from_device(device)?
            .promisc(true)
            .snaplen(5000)
            .timeout(1000)
            .open()?;

        let mut devices = DeviceInfoList::new();

        while let Ok(packet) = cap.next_packet() {
            if let Some(info) = parsers::handle_packet(packet.data) {
                let already_present = if let Some(mac) = info.mac_addr() {
                    devices.find_by_mac(mac).is_some()
                } else {
                    false
                };

                if !already_present {
                    devices.push(info.clone());
                    // Send to UI thread
                    if tx.send(info).is_err() {
                        // Channel closed, UI probably quit
                        break;
                    }
                }
            }
        }

        Ok(())
    }
}
