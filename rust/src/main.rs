mod display;
mod protocol;

use std::env;
use std::io::{Result, ErrorKind, BufRead, BufReader};
use std::thread::sleep;
use std::time::Duration;

use serialport::SerialPort;

// index in array is the id of the display
fn main() -> Result<()> {
    let serial = env::var("SERIAL").unwrap_or(String::from("/dev/ttyUSB0"));

    let mut port = serialport::new(serial, 9_600)
        .timeout(Duration::from_secs(1))
        .open()?;

    println!("Serial open, starting");

    let mut monitor = display::get_display().expect("Error while trying to get display!");
    let mut current_brightness: i32 = -1;

    loop {
        let buffer = match fetch_brightness(port.as_mut()) {
            Ok(buffer) => buffer,
            Err(error) if error.kind() == ErrorKind::TimedOut => {
                println!("Read timed out");
                sleep(Duration::from_secs(1));
                continue;
            },
            Err(error) => {
                return Err(error);
            }
        };

        match protocol::parse_line(buffer) {
            Some(brightness) => {
                if i32::from(brightness).abs_diff(current_brightness) > 1 {
                    println!("Setting to {}", brightness);
                    let _ = display::set_brightness(&mut monitor, brightness);
                    current_brightness = brightness.into();
                } else {
                    sleep(Duration::from_secs(1));
                }
            }
            _ => println!("Cannot parse line"),
        }
    }
}

fn fetch_brightness(port: &mut dyn SerialPort) -> Result<String> {
    let mut buffer = String::new();

    port.flush()?;
    port.write("?".as_bytes())?;
    BufReader::new(port).read_line(&mut buffer)?;

    Ok(buffer)
}
