use etherparse::{PacketHeaders, EtherType, LinkHeader};
mod arp;

pub fn handle_packet(packet_data: &[u8]) {
    match PacketHeaders::from_ethernet_slice(packet_data) {
        Ok(headers) => {
            if let Some(LinkHeader::Ethernet2(eth_header)) = headers.link {
                match eth_header.ether_type {
                    EtherType::ARP => {
                        arp::handle_arp(packet_data);
                    },
                    _ => {}
                }
            }
        }
        Err(_) => {},
    }
}
