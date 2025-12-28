use super::device_info::DeviceInfo;

#[derive(Debug, Default, Clone)]
pub struct DeviceInfoList {
    devices: Vec<DeviceInfo>,
}

impl DeviceInfoList {
    pub fn new() -> Self { Self { devices: Vec::new() } }

    pub fn push(&mut self, info: DeviceInfo) { self.devices.push(info); }

    pub fn len(&self) -> usize { self.devices.len() }
    pub fn is_empty(&self) -> bool { self.devices.is_empty() }

    pub fn iter(&self) -> std::slice::Iter<'_, DeviceInfo> { self.devices.iter() }
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, DeviceInfo> { self.devices.iter_mut() }

    pub fn find_by_name(&self, name: &str) -> Option<&DeviceInfo> {
        self.devices.iter().find(|d| d.name() == name)
    }
    pub fn find_by_name_mut(&mut self, name: &str) -> Option<&mut DeviceInfo> {
        self.devices.iter_mut().find(|d| d.name() == name)
    }

    pub fn find_by_mac(&self, mac: [u8; 6]) -> Option<&DeviceInfo> {
        self.devices.iter().find(|d| d.mac() == Some(mac))
    }

    pub fn as_slice(&self) -> &[DeviceInfo] { &self.devices }
}

impl std::fmt::Display for DeviceInfoList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DeviceInfoList(len={})", self.devices.len())?;
        for d in &self.devices {
            writeln!(f, "  {}", d)?;
        }
        Ok(())
    }
}
