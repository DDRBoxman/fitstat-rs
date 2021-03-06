extern crate fitstat;

use fitstat::FitStatDevice;
use std::{thread, time};

fn main() {
    let ten_millis = time::Duration::from_millis(1000);

    match FitStatDevice::get_serial_numbers() {
        Ok(serials) => {
            for serial in serials {
                println!("{}", serial)
            }
        }
        Err(e) => println!("{}", e)
    }

    let mut device = FitStatDevice::open_first().expect("Failed to open device.");

    device
        .fade_to_rgb(255, 0, 0)
        .expect("Failed to set color.");

    thread::sleep(ten_millis);

    device
        .fade_off()
        .expect("Failed to set color.");
}