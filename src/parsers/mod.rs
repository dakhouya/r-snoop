use etherparse::{PacketHeaders, EtherType, LinkHeader};
mod arp;
use crate::capture::device_info::DeviceInfo;

pub fn handle_packet(packet_data: &[u8]) -> Option<DeviceInfo> {
    match PacketHeaders::from_ethernet_slice(packet_data) {
        Ok(headers) => {
            if let Some(LinkHeader::Ethernet2(eth_header)) = headers.link
                && eth_header.ether_type == EtherType::ARP {
                return arp::handle_arp(packet_data);
            }
            None
        }
        Err(_) => { None },
    }
}
