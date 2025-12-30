use oui_data;

// Device information container for a network interface
#[derive(Debug, Clone, Default)]
pub struct DeviceInfo {
    // DNS name associated with the discovered host (not the monitored interface)
    mac_vendor: String,
    mac_addr: Option<[u8; 6]>,
    ipv4_addrs: Vec<[u8; 4]>,
    ipv6_addrs: Vec<[u8; 16]>,
}

const DEFAULT_MAC_VENDOR: &str = "Unknown";

impl DeviceInfo {
    pub fn new() -> Self {
        Self {
            mac_vendor: DEFAULT_MAC_VENDOR.to_string(),
            mac_addr: None,
            ipv4_addrs: Vec::new(),
            ipv6_addrs: Vec::new(),
        }
    }

    pub fn set_mac(mut self, mac: [u8; 6]) -> Self {
        self.mac_addr = Some(mac);
        let mac_str = format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]
        );
        self.mac_vendor = oui_data::lookup(&mac_str)
            .map(|rec| rec.organization().to_string())
            .unwrap_or(DEFAULT_MAC_VENDOR.to_string());
        self
    }

    pub fn add_ipv4(&mut self, ip: [u8; 4]) {
        self.ipv4_addrs.push(ip);
    }

    pub fn add_ipv6(&mut self, ip: [u8; 16]) {
        self.ipv6_addrs.push(ip);
    }

    pub fn mac_vendor(&self) -> &str {
        &self.mac_vendor
    }
    pub fn mac_addr(&self) -> Option<[u8; 6]> {
        self.mac_addr
    }
    pub fn ipv4(&self) -> &[[u8; 4]] {
        &self.ipv4_addrs
    }
    pub fn ipv6(&self) -> &[[u8; 16]] {
        &self.ipv6_addrs
    }
}

impl std::fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mac_str = self
            .mac_addr
            .map(|m| {
                format!(
                    "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                    m[0], m[1], m[2], m[3], m[4], m[5]
                )
            })
            .unwrap_or_else(|| "unknown".to_string());
        let ipv4_str = if self.ipv4_addrs.is_empty() {
            "[]".to_string()
        } else {
            let parts: Vec<String> = self
                .ipv4_addrs
                .iter()
                .map(|ip| format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]))
                .collect();
            format!("[{}]", parts.join(", "))
        };
        let ipv6_str = if self.ipv6_addrs.is_empty() {
            "[]".to_string()
        } else {
            let parts: Vec<String> = self
                .ipv6_addrs
                .iter()
                .map(|ip| {
                    // Format as standard hex groups
                    let mut s = String::new();
                    for (i, chunk) in ip.chunks(2).enumerate() {
                        let val = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
                        if i > 0 {
                            s.push(':');
                        }
                        s.push_str(&format!("{:x}", val));
                    }
                    s
                })
                .collect();
            format!("[{}]", parts.join(", "))
        };
        write!(
            f,
            "DeviceInfo(mac_vendor={}, mac={}, ipv4={}, ipv6={})",
            self.mac_vendor, mac_str, ipv4_str, ipv6_str
        )
    }
}
