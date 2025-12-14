use etherparse::{PacketHeaders, EtherType, LinkHeader, TransportHeader};
mod arp;
mod mdns;

pub fn handle_packet(packet_data: &[u8]) {
    match PacketHeaders::from_ethernet_slice(packet_data) {
        Ok(headers) => {
            if let Some(LinkHeader::Ethernet2(eth_header)) = headers.link {
                match eth_header.ether_type {
                    EtherType::ARP => {
                        arp::handle_arp(packet_data);
                    },
                    EtherType::IPV4 => {
                        if let Some(TransportHeader::Udp(udp_header)) = headers.transport {
                            if udp_header.source_port == 5353 || udp_header.destination_port == 5353 {
                                mdns::handle_mdns(packet_data);
                            }
                        }
                    },
                    _ => {}
                }
            }
        }
        Err(_) => {},
    }
}
