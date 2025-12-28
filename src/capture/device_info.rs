// Device information container for a network interface
#[derive(Debug, Clone, Default)]
pub struct DeviceInfo {
    // DNS name associated with the discovered host (not the monitored interface)
    dsn_name: String,
    mac_addr: Option<[u8; 6]>,
    ipv4_addrs: Vec<[u8; 4]>,
    ipv6_addrs: Vec<[u8; 16]>,
}

impl DeviceInfo {
    pub fn new<S: Into<String>>(dsn_name: S) -> Self {
        Self {
            dsn_name: dsn_name.into(),
            mac_addr: None,
            ipv4_addrs: Vec::new(),
            ipv6_addrs: Vec::new(),
        }
    }

    pub fn set_mac(mut self, mac: [u8; 6]) -> Self {
        self.mac_addr = Some(mac);
        self
    }

    pub fn add_ipv4(&mut self, ip: [u8; 4]) {
        self.ipv4_addrs.push(ip);
    }

    pub fn add_ipv6(&mut self, ip: [u8; 16]) {
        self.ipv6_addrs.push(ip);
    }

    pub fn name(&self) -> &str {
        &self.dsn_name
    }
    pub fn mac(&self) -> Option<[u8; 6]> {
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
            "DeviceInfo(dsn_name={}, mac={}, ipv4={}, ipv6={})",
            self.dsn_name, mac_str, ipv4_str, ipv6_str
        )
    }
}
