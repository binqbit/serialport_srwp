use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use super::{DeviceError, SerialPortDataManager};

const SRWP_CMD: u8 = 0x00;
const CMD_TEST: u8 = 0x00;
const CMD_READ: u8 = 0x01;
const CMD_WRITE: u8 = 0x02;
const MAX_BYTES_PER_SECOND: usize = 9600 / 8;
const MAX_BYTES_PER_TRANSACTION: usize = 32;
const MAX_TIME_PER_TRANSACTION: u64 =
    (1000 * MAX_BYTES_PER_TRANSACTION / MAX_BYTES_PER_SECOND) as u64;
const MAX_ZERO_READ_COUNT: u32 = 10;

pub trait AddressedIo {
    fn read_data(&mut self, address: u32, size: usize) -> Result<Vec<u8>, DeviceError>;
    fn write_data(&mut self, address: u32, data: &[u8]) -> Result<(), DeviceError>;
}

impl SerialPortDataManager {
    pub fn test(&mut self, data: &[u8]) -> Result<Vec<u8>, DeviceError> {
        let serial_port = self.get_serial_port()?;
        serial_port.clear()?;

        let mut buffer = vec![0u8; 6 + data.len()];
        buffer[0] = SRWP_CMD;
        buffer[1] = CMD_TEST;
        buffer[2..6].copy_from_slice(&(data.len() as u32).to_le_bytes());
        buffer[6..].copy_from_slice(data);

        serial_port.write_data_terminal_ready(true)?;
        serial_port.write_request_to_send(true)?;
        serial_port.write(&buffer)?;
        serial_port.flush()?;
        serial_port.write_request_to_send(false)?;
        serial_port.write_data_terminal_ready(false)?;

        let mut data = vec![0u8; data.len()];
        serial_port.read(&mut data)?;
        Ok(data)
    }

    fn _read_data(&mut self, address: u32, length: usize) -> Result<Vec<u8>, DeviceError> {
        let serial_port = self.get_serial_port()?;
        serial_port.clear()?;

        let mut buffer = vec![0u8; 10];
        buffer[0] = SRWP_CMD;
        buffer[1] = CMD_READ;
        buffer[2..6].copy_from_slice(&address.to_le_bytes());
        buffer[6..10].copy_from_slice(&(length as u32).to_le_bytes());

        serial_port.write_data_terminal_ready(true)?;
        serial_port.write_request_to_send(true)?;
        serial_port.write(&buffer)?;
        serial_port.flush()?;
        serial_port.write_request_to_send(false)?;
        serial_port.write_data_terminal_ready(false)?;
        serial_port.read_data_set_ready()?;

        let mut zero_read_count = 0u32;
        let mut count = 0usize;
        let mut data = vec![0u8; length];
        while count < data.len() {
            let size = serial_port.read(&mut data[count..])?;
            count += size;
            if size == 0 {
                zero_read_count += 1;
                if zero_read_count > MAX_ZERO_READ_COUNT {
                    return Err(DeviceError::IOError(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "No data received from device",
                    )));
                }
            } else {
                zero_read_count = 0;
            }
        }
        Ok(data)
    }

    fn _write_data(&mut self, address: u32, data: &[u8]) -> Result<(), DeviceError> {
        let serial_port = self.get_serial_port()?;
        serial_port.clear()?;

        let mut buffer = vec![0u8; 10 + data.len()];
        buffer[0] = SRWP_CMD;
        buffer[1] = CMD_WRITE;
        buffer[2..6].copy_from_slice(&address.to_le_bytes());
        buffer[6..10].copy_from_slice(&(data.len() as u32).to_le_bytes());
        buffer[10..].copy_from_slice(data);

        serial_port.write_data_terminal_ready(true)?;
        serial_port.write_request_to_send(true)?;
        serial_port.write(&buffer)?;
        serial_port.flush()?;
        serial_port.write_request_to_send(false)?;
        serial_port.write_data_terminal_ready(false)?;
        Ok(())
    }
}

impl AddressedIo for SerialPortDataManager {
    fn read_data(&mut self, address: u32, length: usize) -> Result<Vec<u8>, DeviceError> {
        let mut data = Vec::new();
        let mut count = 0usize;
        while count < length {
            let start_time = Instant::now();
            let size = std::cmp::min(length - count, MAX_BYTES_PER_TRANSACTION);
            let data_part = self._read_data(address + count as u32, size)?;
            data.extend_from_slice(&data_part);
            count += size;
            let elapsed = start_time.elapsed().as_millis() as u64;
            if elapsed < MAX_TIME_PER_TRANSACTION {
                sleep(Duration::from_millis(MAX_TIME_PER_TRANSACTION - elapsed));
            }
        }
        Ok(data)
    }

    fn write_data(&mut self, address: u32, data: &[u8]) -> Result<(), DeviceError> {
        let mut count = 0usize;
        while count < data.len() {
            let start_time = Instant::now();
            let size = std::cmp::min(data.len() - count, MAX_BYTES_PER_TRANSACTION);
            self._write_data(address + count as u32, &data[count..count + size])?;
            count += size;
            let elapsed = start_time.elapsed().as_millis() as u64;
            if elapsed < MAX_TIME_PER_TRANSACTION {
                sleep(Duration::from_millis(MAX_TIME_PER_TRANSACTION - elapsed));
            }
        }
        Ok(())
    }
}
