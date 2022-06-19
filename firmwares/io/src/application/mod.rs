// ============================================================================

/// Number of io on the rp2040
pub const NB_IO_RP2040: usize = 27;
pub const MAX_IO_INDEX_RP2040: usize = 28;

// HAL
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal;
use rp_pico::hal::gpio::dynpin::DynPin;

// USB crates
// use rp_pico::hal::usb::UsbBus;
// use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::UsbDevice;
// use usb_device::prelude::UsbDeviceBuilder;
// use usb_device::prelude::UsbVidPid;

// USB Communications Class Device support
use usbd_serial::SerialPort;




use numtoa::NumToA;


// ============================================================================

use serde::{Deserialize, Serialize};
use serde_json_core;

#[derive(Serialize, Deserialize, Debug)]
struct Command {
    // 0 set mode / 1 write val / 2 read val
    cmd: u8,
    // id of the pin (X => gpioX)
    pin: u8,
    // if cmd = 0 { 0 mode input, 1 mode output }
    arg: u8
}

// ============================================================================

mod buffer;
use buffer::UsbBuffer;

// ============================================================================

/// Store all the usefull objects for the application
pub struct PicohaIo
{
    /// To manage delay
    delay: cortex_m::delay::Delay,

    /// Objects to control io of the board
    dyn_ios: [DynPin; NB_IO_RP2040],
    /// Map to convert gpio index into *dyn_ios* index
    /// This is because some gpioX does not exist (or not driveable) and create hole in the array
    map_ios: [u8; MAX_IO_INDEX_RP2040],

    /// The USB Device Driver (shared with the interrupt).
    usb_device: &'static mut UsbDevice<'static, hal::usb::UsbBus>,

    /// The USB Serial Device Driver (shared with the interrupt).
    usb_serial: &'static mut SerialPort<'static, hal::usb::UsbBus>,

    ///
    usb_buffer: UsbBuffer<1024>,
}

// ============================================================================

/// Implementation of the App
impl PicohaIo
{
    /// Application intialization
    pub fn new(
        delay: cortex_m::delay::Delay,
        dyn_ios: [DynPin; NB_IO_RP2040],
        map_ios: [u8; MAX_IO_INDEX_RP2040],
        usb_dev: &'static mut UsbDevice<'static, hal::usb::UsbBus>,
        usb_ser: &'static mut SerialPort<'static, hal::usb::UsbBus>,
    ) -> Self {
        Self {
            delay: delay,
            dyn_ios: dyn_ios,
            map_ios: map_ios,
            usb_device: usb_dev,
            usb_serial: usb_ser,
            usb_buffer: UsbBuffer::new(),
        }
    }

    /// Main loop of the main task of the application
    pub fn run_forever(&mut self) -> ! {

        // self.usb_serial.write(b"{ \"log\": \"+++ firmware start +++\" }\r\n").ok();

        let mut cmd_buffer = [0u8; 1024];
        loop {
            let mut tmp_buf = [0u8; 20];

            match self.usb_buffer.get_command(&mut cmd_buffer) {
                None => {}
                Some(cmd_end_index) => {
                    let cmd_slice_ref = &cmd_buffer[0..cmd_end_index];

                    match serde_json_core::de::from_slice::<Command>(cmd_slice_ref) {
                        Err(_e) => {
                            // Do nothing
                            let _ = self.usb_serial.write(b"error parsing json command\n");
                            let _ = self.usb_serial.write(cmd_slice_ref);
                            let _ = self.usb_serial.write(b" == ");
                            let _ = self
                                .usb_serial
                                .write(cmd_end_index.numtoa(10, &mut tmp_buf));
                            let _ = self.usb_serial.write(b" == \r\n");
                        }
                        Ok(cmd) => {
                            // // let _ = self.usb_serial.write(cmd.0.cmd.len().numtoa(10, &mut tmp_buf));
                            // let _ = self.usb_serial.write(cmd.0.cmd.as_bytes());
                            // let _ = self.usb_serial.write(b"\n");

                            // let data = &cmd.0;
                            // match data.cmd {
                                
                                // "twi_m_w" => {
                                //     let mut write_data = [0u8; 512];
                                //     match base64::decode_config_slice(
                                //         &data.data,
                                //         base64::STANDARD,
                                //         &mut write_data,
                                //     ) {
                                //         Err(_e) => {}
                                //         Ok(count) => {
                                //             self.i2c
                                //                 .write(
                                //                     0x53,
                                //                     &write_data[..count]
                                //                 )
                                //                 .ok();
                                //         }
                                //     }
                            
                                // }
                                // "twi_m_wr" => {
                                
                                //     let mut write_data = [0u8; 512];
                                //     match base64::decode_config_slice(
                                //         &data.data,
                                //         base64::STANDARD,
                                //         &mut write_data,
                                //     ) {
                                //         Err(_e) => {}
                                //         Ok(count) => {
                                //             let mut readbuf = [0u8; 512];
                                //             self.i2c
                                //                 .write_read(
                                //                     0x53,
                                //                     &write_data[..count],
                                //                     &mut readbuf[..data.size],
                                //                 )
                                //                 .ok();

                                //             // self.usb_serial
                                //             //     .write(write_data[0].numtoa(10, &mut tmp_buf))
                                //             //     .ok();
                                //             // self.usb_serial.write(b" c\n").ok();
                                //             // self.usb_serial
                                //             //     .write(readbuf[0].numtoa(10, &mut tmp_buf))
                                //             //     .ok();
                                //             // self.usb_serial.write(b" c\n").ok();
                                //         }
                                //     }
                                // }
                                // default => {
                                //     self.usb_serial.write(b"{\"log\": \"").ok();
                                //     self.usb_serial.write(default.as_bytes()).ok();
                                //     self.usb_serial.write(b" command not found\"}\r\n").ok();
                                // }
                            // }

                            // let s = b"hello internet!";
                            // let mut buf = [0u8; 150];
                            // // make sure we'll have a slice big enough for base64 + padding
                            // // buf.resize(s.len() * 4 / 3 + 4, 0);
                            // let bytes_written =
                            //     base64::encode_config_slice(s, base64::STANDARD, &mut buf);
                            // let _ = self.usb_serial.write(&buf[0..bytes_written]);
                            // let _ = self.usb_serial.write(b"\n");
                        }
                    }
                }
            }

            // self.led_pin.set_high().ok();
            // self.delay.delay_ms(500);
            // self.led_pin.set_low().ok();
            // self.delay.delay_ms(500);
        }
    }
}

// Method Implementations
mod panic;
mod usbctrl;
