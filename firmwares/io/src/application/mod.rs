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

#[derive(Deserialize, Debug)]
struct Command {
    // 0 set mode / 1 write val / 2 read val
    cod: u8,
    // id of the pin (X => gpioX)
    pin: u8,
    // if cmd = 0 { 0 mode input, 1 mode output }
    arg: u8
}

#[derive(Serialize, Debug)]
struct Answer<'a>  {
    /// Status code
    sts: u8,
    // id of the pin (X => gpioX)
    pin: u8,
    // if cmd = 0 { 0 mode input, 1 mode output }
    arg: u8,
    /// Text message
    msg: &'a str
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
    map_ios: [usize; MAX_IO_INDEX_RP2040],

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
        map_ios: [usize; MAX_IO_INDEX_RP2040],
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

    /// To sned log to the user
    /// 
    pub fn send_log(&mut self) {

        let ans = Answer { sts:0, pin: 0, arg: 0, msg: "truc" };
        let mut tmp_buf = [0u8; 40];
        let j = serde_json_core::to_slice(&ans, &mut tmp_buf);
        
        self.usb_serial.write(&tmp_buf).unwrap();
        self.usb_serial.write(b" == \r\n").unwrap();
    }

    ///
    /// 
    fn process_set_io_mode(&mut self, cmd : &Command ) {

        // Get io from cmd
        let idx = self.map_ios[cmd.pin as usize];
        let io = &mut self.dyn_ios[idx];


        // Configure the pin to operate as a readable push pull output
        io.into_readable_output();

        // Configure the pin to operate as a pulled down input
        // io.into_pull_down_input();
        // io.into_pull_up_input();

    }

    ///
    /// 
    fn process_write_io(&mut self, cmd : &Command ) {

        // Get io from cmd
        let idx = self.map_ios[cmd.pin as usize];
        let io = &mut self.dyn_ios[idx];
        
    }

    ///
    /// 
    fn process_read_io(&mut self, cmd : &Command ) {

        // Get io from cmd
        let idx = self.map_ios[cmd.pin as usize];
        let io = &mut self.dyn_ios[idx];
        
    }

    /// Main loop of the main task of the application
    /// 
    pub fn run_forever(&mut self) -> ! {


        let mut cmd_buffer = [0u8; 1024];
        loop {
            let mut tmp_buf = [0u8; 20];

            match self.usb_buffer.get_command(&mut cmd_buffer) {
                None => {}
                Some(cmd_end_index) => {
                    let cmd_slice_ref = &cmd_buffer[0..cmd_end_index];

                    match serde_json_core::de::from_slice::<Command>(cmd_slice_ref) {
                        
                        Err(_e) => {
                            

                            self.send_log();


                            // // Do nothing
                            // let _ = self.usb_serial.write(b"error parsing json command\n");
                            // let _ = self.usb_serial.write(cmd_slice_ref);
                            // let _ = self.usb_serial.write(b" == ");
                            // let _ = self
                            //     .usb_serial
                            //     .write(cmd_end_index.numtoa(10, &mut tmp_buf));
                            // let _ = self.usb_serial.write(b" == \r\n");
                        }

                        Ok(cmd) => {
                            // let _ = self.usb_serial.write(cmd.0.cmd.len().numtoa(10, &mut tmp_buf));
                        //     let _ = self.usb_serial.write(cmd.0.cmd.numtoa(10, &mut tmp_buf));
                        //     let _ = self.usb_serial.write(b"\n");

                            let data = &cmd.0;
                            match data.cod {

                                0 => {
                                    self.process_set_io_mode(data);
                                }

                                1 => {
                                    self.process_write_io(data);
                                }

                                2 => {
                                    self.process_read_io(data);
                                }

                                default => {
                                    self.usb_serial.write(b"{\"log\": \"").ok();
                                    // self.usb_serial.write(default.as_bytes()).ok();
                                    self.usb_serial.write(b" command not found\"}\r\n").ok();
                                }
                            }
                                

                        }
                    }
                }
            }

        }
    }
}

// Method Implementations
mod panic;
mod usbctrl;
