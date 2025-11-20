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
    let handle: DeviceHandle<rusb::GlobalContext> = match rusb::open_device_with_vid_pid(VID, PID) {
        Some(h) => h,
        None => {
            eprintln!("Error: Device not found");
            return Ok(());
        }
    };

    handle.claim_interface(0);

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

    handle.release_interface(0);

    println!("Done.");

    Ok(())
}





// use rusb::{DeviceHandle, Direction, Recipient, RequestType, UsbContext};
// use std::time::Duration;

// const VID: u16 = 0x2886;
// const PID: u16 = 0x0018;
// const TIMEOUT: Duration = Duration::from_millis(8000);

// const REQUEST_TYPE: u8 = rusb::request_type(
//     Direction::Out,
//     RequestType::Vendor,
//     Recipient::Device,
// );

// // 1. Create a helper struct to handle the "cleanup" automatically (RAII pattern)
// struct KernelDriverGuard<'a> {
//     handle: &'a DeviceHandle<rusb::GlobalContext>,
//     interface_number: u8,
//     was_active: bool,
// }

// impl<'a> KernelDriverGuard<'a> {
//     fn new(handle: &'a DeviceHandle<rusb::GlobalContext>, interface_number: u8) -> Self {
//         let was_active = handle.kernel_driver_active(interface_number).unwrap_or(false);
        
//         if was_active {
//             println!("Detaching kernel driver for interface {}...", interface_number);
//             let _ = handle.detach_kernel_driver(interface_number);
//         }

//         KernelDriverGuard {
//             handle,
//             interface_number,
//             was_active,
//         }
//     }
// }

// // This runs automatically when the guard goes out of scope (end of main or panic)
// impl<'a> Drop for KernelDriverGuard<'a> {
//     fn drop(&mut self) {
//         if self.was_active {
//             println!("Re-attaching kernel driver for interface {}...", self.interface_number);
//             let _ = self.handle.attach_kernel_driver(self.interface_number);
//         }
//     }
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     println!("Looking for device {:04x}:{:04x}...", VID, PID);

//     let handle = match rusb::open_device_with_vid_pid(VID, PID) {
//         Some(h) => h,
//         None => {
//             eprintln!("Error: Device not found");
//             return Ok(());
//         }
//     };

//     // 2. Initialize the guard. This detaches the driver immediately.
//     // Note: 0 is the usual interface, but if your device is complex, 
//     // check which interface controls the function you are targeting.
//     let _guard = KernelDriverGuard::new(&handle, 0);

//     let data = [0u8];

//     println!("Sending Control Transfer 1 (0x20)...");
//     handle.write_control(
//         REQUEST_TYPE,
//         0,
//         0x20,
//         0x1C,
//         &data,
//         TIMEOUT,
//     )?;

//     println!("Sending Control Transfer 2 (0x22)...");
//     handle.write_control(
//         REQUEST_TYPE,
//         0,
//         0x22,
//         0x1C,
//         &data,
//         TIMEOUT,
//     )?;

//     println!("Done.");
    
//     // When _guard goes out of scope here, it automatically re-attaches the kernel driver.
//     Ok(())
// }