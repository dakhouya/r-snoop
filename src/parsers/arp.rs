use etherparse::{PacketHeaders};
use oui_data;

pub fn handle_arp(packet: &[u8]) {
    match PacketHeaders::from_ethernet_slice(packet) {
        Ok(headers) => {
            if let Some(etherparse::NetHeaders::Arp(arp)) = headers.net {
                let hw_addr_len = arp.hw_addr_size() as usize;
                let proto_addr_len = arp.protocol_addr_size() as usize;
                let sender_hw_addr = &arp.sender_hw_addr()[..hw_addr_len];
                let mac_str = format!(
                    "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                    sender_hw_addr[0], sender_hw_addr[1], sender_hw_addr[2],
                    sender_hw_addr[3], sender_hw_addr[4], sender_hw_addr[5]
                );
                let sender_proto_addr = &arp.sender_protocol_addr()[..proto_addr_len];
                let sender_proto_addr_str = format!(
                    "{}.{}.{}.{}",
                    sender_proto_addr[0], sender_proto_addr[1], sender_proto_addr[2], sender_proto_addr[3]
                );
                match oui_data::lookup(&mac_str) {
                    Some(record) => {
                        println!("[ARP] [{}] - {} - {}", mac_str, sender_proto_addr_str, record.organization());
                    }
                    None => {
                        println!("[ARP] [{}] - {}", mac_str, sender_proto_addr_str);
                    }
                }
            } else {
                println!("[ARP] No ARP header found in the packet.");
            }
        },
        Err(e) => {
            println!("[ARP] Failed to parse packet headers: {}", e);
        }
    }
}
