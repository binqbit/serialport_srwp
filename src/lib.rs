use std::sync::{Arc, Mutex};

use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPort, StopBits};

#[derive(Debug)]
pub enum DeviceError {
    NotFound,
    SerialPortError(serialport::Error),
    IOError(std::io::Error),
    PortIsLocked,
}

#[derive(Debug)]
pub struct SerialPortBox {
    port: Box<dyn SerialPort>,
}

#[derive(Debug)]
pub struct SerialPortDataManager(Arc<Mutex<SerialPortBox>>);

impl SerialPortDataManager {
    pub fn new(path: &str) -> Result<Self, DeviceError> {
        serialport::new(path, 9600)
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .open()
            .map(|port| SerialPortBox { port })
            .map(Mutex::new)
            .map(Arc::new)
            .map(SerialPortDataManager)
            .map_err(DeviceError::SerialPortError)
    }

    pub fn get_available_ports() -> Result<Vec<String>, DeviceError> {
        serialport::available_ports()
            .map_err(DeviceError::SerialPortError)
            .map(|ports| {
                ports
                    .iter()
                    .filter_map(|port| {
                        if let serialport::SerialPortType::UsbPort(_) = &port.port_type {
                            Some(port.port_name.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .and_then(|ports| {
                if ports.is_empty() {
                    Err(DeviceError::NotFound)
                } else {
                    Ok(ports)
                }
            })
    }

    pub fn get_serial_port(&mut self) -> Result<&mut SerialPortBox, DeviceError> {
        Arc::get_mut(&mut self.0)
            .ok_or(DeviceError::PortIsLocked)?
            .get_mut()
            .map_err(|_| DeviceError::PortIsLocked)
    }
}

impl Clone for SerialPortDataManager {
    fn clone(&self) -> Self {
        SerialPortDataManager(Arc::clone(&self.0))
    }
}

impl SerialPortBox {
    pub fn read_data_set_ready(&mut self) -> Result<bool, DeviceError> {
        self.port
            .read_data_set_ready()
            .map_err(DeviceError::SerialPortError)
    }

    pub fn write_data_terminal_ready(&mut self, ready: bool) -> Result<(), DeviceError> {
        self.port
            .write_data_terminal_ready(ready)
            .map_err(DeviceError::SerialPortError)
    }

    pub fn write_request_to_send(&mut self, ready: bool) -> Result<(), DeviceError> {
        self.port
            .write_request_to_send(ready)
            .map_err(DeviceError::SerialPortError)
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), DeviceError> {
        self.port.write_all(data).map_err(DeviceError::IOError)
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize, DeviceError> {
        self.port.read(buffer).map_err(DeviceError::IOError)
    }

    pub fn clear(&mut self) -> Result<(), DeviceError> {
        self.port
            .clear(ClearBuffer::All)
            .map_err(DeviceError::SerialPortError)
    }

    pub fn flush(&mut self) -> Result<(), DeviceError> {
        self.port.flush().map_err(DeviceError::IOError)
    }
}

mod srwp;
mod types;

pub mod device;

pub use types::*;
