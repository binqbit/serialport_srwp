# Serial Port Simple Read Write Protocol (SRWP)

## Project Description
This project was developed for use with the Blaustahl Storage Device, providing a simple method for data transfer and memory management of the device via a serial port using the Simple Read Write Protocol (SRWP) API. The protocol implements basic read and write operations, ensuring reliable interaction with the device. The example and implementation of this protocol were specifically developed for [this fork of the repository](https://github.com/binqbit/blaustahl), which contains the Blaustahl Storage Device and corresponding drivers.

## Data Transfer Protocol
The SRWP protocol allows interaction with the Blaustahl Storage Device through a serial port. It uses commands, each defined by a unique code, and includes a zero byte indicating that it is an SRWP command.

### Protocol Commands
1. **CMD_TEST (0x00):** Used for testing data transmission. The command sends data to the device and expects it to be returned. Returns bytes of data received from the device.
2. **CMD_READ (0x01):** Reads data from the device. Requires specifying the address and the length of the data to be read. Returns bytes of data read from the specified address.
3. **CMD_WRITE (0x02):** Writes data to the device. Requires specifying the address and the data to be written. This command does not return any data.

### Command Transmission Format
Each command begins with a zero byte indicating an SRWP command and follows the format:

- **SRWP\_CMD (0x00):** Indicates the start of an SRWP command.

- **CMD_TEST:**
  - `SRWP_CMD` - Zero byte indicating an SRWP command.
  - `CMD_TEST` - Command code.
  - `<data length>` - 4 bytes indicating the length of the data.
  - `<data>` - Bytes of data to be transmitted.
  - **Returns:** Bytes of data sent back from the device.

- **CMD_READ:**
  - `SRWP_CMD` - Zero byte indicating an SRWP command.
  - `CMD_READ` - Command code.
  - `<address>` - 4 bytes indicating the starting address for reading.
  - `<length>` - 4 bytes indicating the length of the data to be read.
  - **Returns:** Bytes of data read from the specified address.

- **CMD_WRITE:**
  - `SRWP_CMD` - Zero byte indicating an SRWP command.
  - `CMD_WRITE` - Command code.
  - `<address>` - 4 bytes indicating the starting address for writing.
  - `<data length>` - 4 bytes indicating the length of the data to be written.
  - `<data>` - Bytes of data to be written.
  - **Returns:** This command does not return any data.

## SRWP Protocol Description
The SRWP protocol is a simple and effective method for memory management of the Blaustahl Storage Device via a serial port. It supports basic read and write operations, ensuring reliable interaction between the device and the host. The example code demonstrates the use of the `serialport` library to implement the protocol.

SRWP uses various commands to manage data transfer, making it a flexible and easy-to-use solution for applications requiring reliable data transmission.

## Getting Started
To get started with the SRWP protocol, follow these steps:

1. Connect the Blaustahl Storage Device to your computer via USB.
2. Ensure the serial port is configured correctly (9600 baud, 8 data bits, 1 stop bit, software flow control, no parity).
3. Use the provided code to perform read and write operations on the device.