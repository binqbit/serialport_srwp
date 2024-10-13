use std::{thread::sleep, time::{Duration, Instant}};

use crate::spdm::SerialPortDataManager;

const SRWP_CMD: u8 = 0x00;
const CMD_TEST: u8 = 0x00;
const CMD_READ: u8 = 0x01;
const CMD_WRITE: u8 = 0x02;
const MAX_BYTES_PER_SECOND: u64 = 9600 / 8;
const MAX_BYTES_PER_TRANSACTION: u32 = 32;
const MAX_TIME_PER_TRANSACTION: u64 = 1000 * MAX_BYTES_PER_TRANSACTION as u64 / MAX_BYTES_PER_SECOND;

impl SerialPortDataManager {
    pub fn test(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        self.clear()?;

        let mut buffer = vec![0u8; 6 + data.len()];
        buffer[0] = SRWP_CMD;
        buffer[1] = CMD_TEST;
        buffer[2..6].copy_from_slice(&(data.len() as u32).to_le_bytes());
        buffer[6..].copy_from_slice(data);

        self.write_data_terminal_ready(true)?;
        self.write_request_to_send(true)?;
        self.write(&buffer)?;
        self.flush()?;
        self.write_request_to_send(false)?;
        self.write_data_terminal_ready(false)?;

        let mut data = vec![0u8; data.len()];
        self.read(&mut data)?;
        Ok(data)
    }

    fn _read_data(&mut self, address: u32, length: u32) -> std::io::Result<Vec<u8>> {
        self.clear()?;

        let mut buffer = vec![0u8; 10];
        buffer[0] = SRWP_CMD;
        buffer[1] = CMD_READ;
        buffer[2..6].copy_from_slice(&address.to_le_bytes());
        buffer[6..10].copy_from_slice(&length.to_le_bytes());

        self.write_data_terminal_ready(true)?;
        self.write_request_to_send(true)?;
        self.write(&buffer)?;
        self.flush()?;
        self.write_request_to_send(false)?;
        self.write_data_terminal_ready(false)?;

        self.read_data_set_ready()?;
        let mut count = 0u32;
        let mut data = vec![0u8; length as usize];
        while count < data.len() as u32 {
            let size = self.read(&mut data[count as usize..])?;
            count += size as u32;
        }
        Ok(data)
    }

    fn _write_data(&mut self, address: u32, data: &[u8]) -> std::io::Result<()> {
        self.clear()?;

        let mut buffer = vec![0u8; 10 + data.len()];
        buffer[0] = SRWP_CMD;
        buffer[1] = CMD_WRITE;
        buffer[2..6].copy_from_slice(&address.to_le_bytes());
        buffer[6..10].copy_from_slice(&(data.len() as u32).to_le_bytes());
        buffer[10..].copy_from_slice(data);

        self.write_data_terminal_ready(true)?;
        self.write_request_to_send(true)?;
        self.write(&buffer)?;
        self.flush()?;
        self.write_request_to_send(false)?;
        self.write_data_terminal_ready(false)?;
        Ok(())
    }

    pub fn read_data(&mut self, address: u32, length: u32) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        let mut count = 0u32;
        while count < length {
            let start_time = Instant::now();
            let size = std::cmp::min(length - count, MAX_BYTES_PER_TRANSACTION);
            let data_part = self._read_data(address + count, size)?;
            data.extend_from_slice(&data_part);
            count += size;
            let elapsed = start_time.elapsed().as_millis() as u64;
            if elapsed < MAX_TIME_PER_TRANSACTION {
                sleep(Duration::from_millis(MAX_TIME_PER_TRANSACTION - elapsed));
            }
        }
        Ok(data)
    }

    pub fn write_data(&mut self, address: u32, data: &[u8]) -> std::io::Result<()> {
        let mut count = 0u32;
        while count < data.len() as u32 {
            let start_time = Instant::now();
            let size = std::cmp::min(data.len() as u32 - count, MAX_BYTES_PER_TRANSACTION);
            self._write_data(address + count, &data[count as usize..count as usize + size as usize])?;
            count += size;
            let elapsed = start_time.elapsed().as_millis() as u64;
            if elapsed < MAX_TIME_PER_TRANSACTION {
                sleep(Duration::from_millis(MAX_TIME_PER_TRANSACTION - elapsed));
            }
        }
        Ok(())
    }
}