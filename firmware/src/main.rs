//! # Pico USB Serial (with Interrupts) Example
//!
//! Creates a USB Serial device on a Pico board, with the USB driver running in
//! the USB interrupt.
//!
//! This will create a USB Serial device echoing anything it receives. Incoming
//! ASCII characters are converted to upercase, so you can tell it is working
//! and not just local-echo!
//!
//! See the `Cargo.toml` file for Copyright and licence details.

#![no_std]
#![no_main]


mod config;

// Define
mod panic;

// The macro for our start-up function
use cortex_m_rt::entry;

// The macro for marking our interrupt functions
use rp_pico::hal::pac::interrupt;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;

// Time handling traits
use embedded_time::rate::*;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
// use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;


/// The USB Serial Device Driver (shared with the interrupt).
static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;


// use heapless::consts::*;
use serde::{Serialize, Deserialize};
use serde_json_core;


#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}


use heapless::spsc::Queue;

// Notice, type signature needs to be explicit for now.
// (min_const_eval, does not allow for default type assignments)
static mut QQQ: Queue<[u8; 100], 4> = Queue::new();



static mut CmdBuffer: [u8; 100] = [0; 100];
static mut CmdBufferIdx: u32 = 0;


/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {


    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }

    // Grab a reference to the USB Bus allocator. We are promising to the
    // compiler not to take mutable access to this global variable whilst this
    // reference exists!
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Set up the USB Communications Class Device driver
    let serial = SerialPort::new(bus_ref);
    unsafe {
        USB_SERIAL = Some(serial);
    }

    // Create a USB device with a fake VID and PID
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(config::USB_MANUFACTURER_ID, config::USB_PRODUCT_ID))
        .manufacturer(config::USB_MANUFACTURER_NAME)
        .product(config::USB_PRODUCT_NAME)
        .serial_number(config::USB_SERIAL_NUMBER)
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    // Enable the USB interrupt
    unsafe {
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };

    // No more USB code after this point in main! We can do anything we want in
    // here since USB is handled in the interrupt - let's blink an LED!

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();


    // NOTE(unsafe) beware of aliasing the `consumer` end point
    let mut consumer = unsafe { QQQ.split().1 };

    // Blink the LED at 1 Hz
    loop {
        // `dequeue` is a lockless operation
        let cmd = consumer.dequeue();
        if cmd != None {
            {
                cortex_m::interrupt::free(|_| {
                    unsafe {
                        let serial = USB_SERIAL.as_mut().unwrap();
                        let _ = serial.write(b"{ \"debug\": \"looop\" }\r\n");
                    }
                })
            }
        }
            

        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}


/// convert int n into a u8 buffer buf
fn base_10_bytes(mut n: u8, buf: &mut [u8]) -> &[u8] {
    if n == 0 {
        return b"0";
    }
    let mut i = 0;
    while n > 0 {
        buf[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }
    let slice = &mut buf[..i];
    slice.reverse();
    &*slice
}


/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // use core::sync::atomic::{AtomicBool, Ordering};

    // Grab the global objects. This is OK as we only access them under interrupt.
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let serial = USB_SERIAL.as_mut().unwrap();


    let point = Point { x: 1, y: 2 };
    let serialized = serde_json_core::to_string::<Point, 100>(&point);

    
    // NOTE(unsafe) beware of aliasing the `producer` end point
    let mut producer = unsafe { QQQ.split().0 };


    // Poll the USB driver with all of our supported USB Classes
    if usb_dev.poll(&mut [serial]) {
        let mut buf = [0u8; 128];
        match serial.read(&mut buf) {
            Err(_e) => {
                // Do nothing
            }
            Ok(0) => {
                // Do nothing
            }
            Ok(count) => {

                


                
                // static mut CmdBuffer: [u8; 100] = [0; 100];
                // static mut CmdBufferIdx: u32 = 0;
                
                // let _ = serial.write(serialized.unwrap().as_bytes());
                

                for i in 0..count {
                    
                    let c = buf[i];

                    let mut bbbb=[0u8;4];
                    let oo = base_10_bytes(c, &mut bbbb);

                    let mut bbbb2=[0u8;4];
                    let oo8 = base_10_bytes('\n' as u8, &mut bbbb2);

                    serial.write(&[c]).unwrap();
                    serial.write(b" ").unwrap();
                    serial.write(&oo).unwrap();
                    serial.write(b" ").unwrap();
                    serial.write(&oo8).unwrap();
                    serial.write(b" \r\n").unwrap();

                    if c == 'z' as u8 {
                        let _ = serial.write(b"{ \"debug\": \"Hello!\" }\r\n");
                    }

                    let mm = b"{ \"debug\": \"Hello return\" }\r\n";
                    for n in 0..mm.len() {
                        CmdBuffer[n] = mm[n];
                    }
                    

                    if c == '\r' as u8 || c == '\n' as u8 {
                        let _ = serial.write(b"{ \"debug\": \"Hello return\" }\r\n");

                        producer.enqueue(CmdBuffer).ok().unwrap();
                    }
                    
                }
                
                // // Convert to upper case
                // buf.iter_mut().take(count).for_each(|b| {
                //     b.make_ascii_uppercase();
                // });

                // // Send back to the host
                // let mut wr_ptr = &buf[..count];
                // while !wr_ptr.is_empty() {
                //     let _ = serial.write(wr_ptr).map(|len| {
                //         wr_ptr = &wr_ptr[len..];
                //     });
                // }
            }
        }
    }
}

// End of file

