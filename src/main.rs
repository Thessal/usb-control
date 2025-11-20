use rusb::{DeviceHandle, Direction, Recipient, RequestType, UsbContext};
use std::time::Duration;

const VID: u16 = 0x2886;
const PID: u16 = 0x0018;
const TIMEOUT: Duration = Duration::from_millis(8000);

const REQUEST_TYPE: u8 = rusb::request_type(
    Direction::Out,
    RequestType::Vendor,
    Recipient::Device,
);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Looking for device {:04x}:{:04x}...", VID, PID);

    // Open the device
    let mut handle: DeviceHandle<rusb::GlobalContext> = match rusb::open_device_with_vid_pid(VID, PID) {
        Some(h) => h,
        None => {
            eprintln!("Error: Device not found");
            return Ok(());
        }
    };

    if let Ok(active) = handle.kernel_driver_active(0) {
        if active {
            let _ = handle.detach_kernel_driver(0);
        }
    }

    let data = [0u8];

    println!("Sending Control Transfer 1 (0x20)...");
    let _bytes_written = handle.write_control(
        REQUEST_TYPE, // bmRequestType
        0,            // bRequest
        0x20,         // wValue
        0x1C,         // wIndex
        &data,        // data buffer
        TIMEOUT,      // timeout
    )?;

    println!("Sending Control Transfer 2 (0x22)...");
    let _bytes_written_2 = handle.write_control(
        REQUEST_TYPE,
        0,
        0x22,
        0x1C,
        &data,
        TIMEOUT,
    )?;

    println!("Done.");
    
    Ok(())
}
