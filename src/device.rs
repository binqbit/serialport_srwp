use super::{DeviceError, SerialPortDataManager};

pub struct SerialPortDevice {
    pub path: String,
}

impl SerialPortDataManager {
    pub fn find_devices() -> Result<Vec<SerialPortDevice>, DeviceError> {
        let ports = SerialPortDataManager::get_available_ports()?;
        Ok(ports.into_iter().map(|port| SerialPortDevice::new(&port)).collect())
    }
}

impl SerialPortDevice {
    pub fn new(path: &str) -> Self {
        SerialPortDevice {
            path: path.to_string(),
        }
    }

    pub fn connect(&self) -> Result<SerialPortDataManager, DeviceError> {
        SerialPortDataManager::new(&self.path)
    }
}
