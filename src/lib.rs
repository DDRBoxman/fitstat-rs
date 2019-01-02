extern crate serialport;

use serialport::SerialPortType;

const FIT_STAT_VENDOR_ID: u16 = 0x2047;
const FIT_STAT_PRODUCT_ID: u16 = 0x03df;
/*
  /dev/tty.usbmodem201
    Type: USB
    VID:2047 PID:03df
     Serial Number: 073BB04620002600
      Manufacturer: Compulab LTD
           Product: fit_StatUSB
           */

pub struct FitStatDevice {
    port: Box<dyn serialport::SerialPort>,
}

impl FitStatDevice {
    pub fn find_first() -> Result<FitStatDevice, serialport::Error> {
        match serialport::available_ports() {
            Ok(ports) => {
                for p in ports {
                    match p.port_type {
                        SerialPortType::UsbPort(info) => {
                            if info.vid == FIT_STAT_VENDOR_ID && info.pid == FIT_STAT_PRODUCT_ID {
                                match serialport::open(&p.port_name) {
                                    Ok(port) => return Ok(FitStatDevice { port: port}),
                                    Err(e) => return Err(e)
                                }
                            }
                        },
                        _ => continue
                    }
                }

                return Err(serialport::Error::new(serialport::ErrorKind::NoDevice, "No fitstat device found"))
            },
            Err(e) => return Err(e)
        }
    }

    pub fn fade_off(&mut self) -> Result<(), std::io::Error> {
        return self.fade_to_rgb(0, 0, 0);
    }

    pub fn fade_to_rgb(&mut self, r: u8, g: u8, b: u8) -> Result<(), std::io::Error> {
        let formatted_message = format!("#{:02X}{:02X}{:02X}\n", r, g, b);
        self.port.write_all(formatted_message.as_bytes())
    }
}
