use serialport::{ClearBuffer, DataBits, FlowControl, Parity, SerialPort, StopBits};

pub struct SerialPortDataManager {
    port: Box<dyn SerialPort>,
}

impl SerialPortDataManager {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let port = serialport::new(path, 9600)
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One)
            .flow_control(FlowControl::None)
            .parity(Parity::None)
            .open()?;
        Ok(SerialPortDataManager { port })
    }

    pub fn read_data_set_ready(&mut self) -> std::io::Result<bool> {
        Ok(self.port.read_data_set_ready()?)
    }

    pub fn write_data_terminal_ready(&mut self, ready: bool) -> std::io::Result<()> {
        self.port.write_data_terminal_ready(ready)?;
        Ok(())
    }

    pub fn write_request_to_send(&mut self, ready: bool) -> std::io::Result<()> {
        self.port.write_request_to_send(ready)?;
        Ok(())
    }

    pub fn write(&mut self, data: &[u8]) -> std::io::Result<()> {
        self.port.write_all(data)?;
        Ok(())
    }

    pub fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
        let bytes_read = self.port.read(buffer)?;
        Ok(bytes_read)
    }

    pub fn clear(&self) -> std::io::Result<()> {
        self.port.clear(ClearBuffer::All)?;
        Ok(())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        self.port.flush()?;
        Ok(())
    }
}

mod device;
mod srwp;
