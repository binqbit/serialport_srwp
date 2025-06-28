use super::{DeviceError, SerialPortDataManager};

pub enum RecordDirection {
    Right,
    Left,
}

impl SerialPortDataManager {
    pub fn read_value<T: Sized>(&mut self, address: u32) -> Result<T, DeviceError> {
        let size = std::mem::size_of::<T>();
        let data = self.read_data(address, size as u32)?;
        let value = unsafe { std::ptr::read(data.as_ptr() as *const T) };
        Ok(value)
    }

    pub fn read_values<T: Sized>(
        &mut self,
        address: u32,
        record_direction: RecordDirection,
    ) -> Result<Vec<T>, DeviceError> {
        let size = self.read_value::<u32>(address)?;
        if size == 0 {
            return Ok(Vec::new());
        }
        let value_size = std::mem::size_of::<T>() as u32;
        let data = match record_direction {
            RecordDirection::Right => self.read_data(address + 4, size * value_size)?,
            RecordDirection::Left => {
                self.read_data(address - size * value_size, size * value_size)?
            }
        };
        let mut result = Vec::new();
        let value_size = value_size as usize;
        for i in 0..size as usize {
            let value = unsafe { std::ptr::read(data[i * value_size..].as_ptr() as *const T) };
            result.push(value);
        }
        Ok(result)
    }

    pub fn write_value<T: Sized>(&mut self, address: u32, value: T) -> Result<(), DeviceError> {
        let size = std::mem::size_of::<T>();
        let data = unsafe { std::slice::from_raw_parts(&value as *const T as *const u8, size) };
        self.write_data(address, data)
    }

    pub fn write_values<T: Sized>(
        &mut self,
        address: u32,
        values: &[T],
        record_direction: RecordDirection,
    ) -> Result<(), DeviceError> {
        self.write_value(address, values.len())?;
        if values.is_empty() {
            return Ok(());
        }
        let value_size = std::mem::size_of::<T>();
        let mut data = Vec::new();
        for i in 0..values.len() {
            let value = unsafe {
                std::slice::from_raw_parts(&values[i] as *const T as *const u8, value_size)
            };
            data.extend_from_slice(value);
        }
        match record_direction {
            RecordDirection::Right => self.write_data(address + 4, &data),
            RecordDirection::Left => {
                self.write_data(address - values.len() as u32 * value_size as u32, &data)
            }
        }
    }
}
