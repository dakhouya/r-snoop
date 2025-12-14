use dns_parser::{Packet, RData};
use std::net::Ipv4Addr;

pub fn handle_mdns(packet: &[u8]) {
    match Packet::parse(packet) {
        Ok(parsed_packet) => {
            println!("[MDNS] Decoded MDNS packet:");

            // Extract destination IP from the packet
            if let Some(dest_ip) = extract_destination_ip(packet) {
                println!("[MDNS] Destination IP: {:?}", dest_ip);

                // Check if the destination IP is 224.0.0.251
                if dest_ip != Ipv4Addr::new(224, 0, 0, 251) {
                    return;
                }

                for question in parsed_packet.questions {
                    println!("[MDNS] Question: {:?}", question);
                }
                for answer in parsed_packet.answers {
                    match &answer.data {
                        RData::A(ipv4_addr) => {
                            println!("[MDNS] Domain: {:?}, IPv4 Address: {:?}", answer.name, ipv4_addr);
                        }
                        RData::AAAA(ipv6_addr) => {
                            println!("[MDNS] Domain: {:?}, IPv6 Address: {:?}", answer.name, ipv6_addr);
                        }
                        _ => {
                            println!("[MDNS] Other Answer: {:?}", answer);
                        }
                    }
                }
            } else {
                println!("[MDNS] Failed to extract destination IP from the packet.");
            }
        }
        Err(dns_parser::Error::LabelIsNotAscii) => {
            println!("[MDNS] Skipping packet: Label contains non-ASCII characters.");
        }
        Err(e) => {
            println!("[MDNS] Failed to decode MDNS packet: {}", e);
        }
    }
}

fn extract_destination_ip(packet: &[u8]) -> Option<Ipv4Addr> {
    // Assuming the destination IP is in the IP header (offset 16-19 for IPv4)
    if packet.len() >= 20 {
        Some(Ipv4Addr::new(packet[16], packet[17], packet[18], packet[19]))
    } else {
        None
    }
}