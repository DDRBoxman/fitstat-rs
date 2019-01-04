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
    fn open_internal(serial_number: Option<&str>) -> Result<FitStatDevice, serialport::Error> {
        match serialport::available_ports() {
            Ok(ports) => {
                for p in ports {
                    match p.port_type {
                        SerialPortType::UsbPort(info) => {
                            if info.vid == FIT_STAT_VENDOR_ID && info.pid == FIT_STAT_PRODUCT_ID {
                                if let Some(wanted_serial_number) = serial_number {
                                    if let Some(p_serial) = info.serial_number {
                                        if  p_serial != wanted_serial_number {
                                            continue
                                        }
                                    }
                                }

                                match serialport::open(&p.port_name) {
                                    Ok(port) => return Ok(FitStatDevice { port }),
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

    pub fn open_first() -> Result<FitStatDevice, serialport::Error> {
        FitStatDevice::open_internal(None)
    }

    pub fn open(serial_number: &str) -> Result<FitStatDevice, serialport::Error> {
        FitStatDevice::open_internal(Some(serial_number))
    }

    pub fn get_serial_numbers() -> Result<Vec<String>, serialport::Error> {
        match serialport::available_ports() {
            Ok(ports) => {
                let mut vec: Vec<String> = Vec::new();

                for p in ports {
                    match p.port_type {
                        SerialPortType::UsbPort(info) => {
                            if info.vid == FIT_STAT_VENDOR_ID && info.pid == FIT_STAT_PRODUCT_ID {
                                if let Some(serial_number) = info.serial_number {
                                    vec.push(serial_number.to_owned());
                                }
                            }
                        },
                        _ => continue
                    }
                }

                return Ok(vec);
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
