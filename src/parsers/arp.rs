use crate::capture::device_info::DeviceInfo;
use etherparse::PacketHeaders;

pub fn handle_arp(packet: &[u8]) -> Option<DeviceInfo> {
    match PacketHeaders::from_ethernet_slice(packet) {
        Ok(headers) => {
            if let Some(etherparse::NetHeaders::Arp(arp)) = headers.net {
                let hw_addr_len = arp.hw_addr_size() as usize;
                let proto_addr_len = arp.protocol_addr_size() as usize;
                let sender_hw_addr = &arp.sender_hw_addr()[..hw_addr_len];
                if sender_hw_addr.len() < 6 || proto_addr_len < 4 {
                    return None;
                }
                let mac_arr = [
                    sender_hw_addr[0],
                    sender_hw_addr[1],
                    sender_hw_addr[2],
                    sender_hw_addr[3],
                    sender_hw_addr[4],
                    sender_hw_addr[5],
                ];
                let sender_proto_addr = &arp.sender_protocol_addr()[..proto_addr_len];
                let ipv4_arr = [
                    sender_proto_addr[0],
                    sender_proto_addr[1],
                    sender_proto_addr[2],
                    sender_proto_addr[3],
                ];

                let mut info = DeviceInfo::new().set_mac(mac_arr);
                info.add_ipv4(ipv4_arr);
                Some(info)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
