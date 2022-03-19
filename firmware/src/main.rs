
#![no_std]
#![no_main]

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
use rp_pico::hal::gpio;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;

// /// The USB Device Driver (shared with the interrupt).
// static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

// /// The USB Bus Driver (shared with the interrupt).
// static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

// /// The USB Serial Device Driver (shared with the interrupt).
// static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;

// I2C HAL traits & Types.
use embedded_hal::blocking::i2c::{Operation, Read, Transactional, Write, WriteRead};

// use heapless::consts::*;
use serde::{Deserialize, Serialize};
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

// ============================================================================

mod application;
mod platform;

// ============================================================================

type TypePinLed = gpio::Pin<gpio::bank0::Gpio25, gpio::Output<gpio::PushPull>>;

// ============================================================================

/// Application object
static mut APP_INSTANCE: Option<application::HostAdapter<TypePinLed>> = None;

/// USB bus allocator
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;
/// USB device object
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;
/// USB serial object
static mut USB_SERIAL: Option<SerialPort<hal::usb::UsbBus>> = None;

// ============================================================================

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
unsafe fn main() -> ! {
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

    USB_SERIAL = Some(platform::init_usb_serial(bus_ref));
    USB_DEVICE = Some(platform::init_usb_device(bus_ref));

    // Enable the USB interrupt
    pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);

    // // Disable the USB interrupt
    // pac::NVIC::mask(hal::pac::Interrupt::USBCTRL_IRQ);


    //
    // No more USB code after this point in main! We can do anything we want in
    // here since USB is handled in the interrupt
    //

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Declare I2C
    // Configure two pins as being I²C, not GPIO
    let sda_pin = pins.gpio20.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio21.into_mode::<hal::gpio::FunctionI2C>();

    // Create the I²C driver, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let mut i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        100.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock,
    );

    let mut readbuf: [u8; 1] = [0; 1];
    i2c.write_read(0x53, &[0], &mut readbuf).unwrap();
    // 0x00 =>  229 (11100101)

    // NOTE(unsafe) beware of aliasing the `consumer` end point
    // let mut consumer = unsafe { QQQ.split().1 };

    // Init the application and start it
    APP_INSTANCE = Some(application::HostAdapter::new(
        cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer()), // Append delay feature to the app
        pins.led.into_push_pull_output(), // Set the led gpio as output
        USB_DEVICE.as_mut().unwrap(),
        USB_SERIAL.as_mut().unwrap(),
    ));

    // Run the application
    let app = APP_INSTANCE.as_mut().unwrap();
    app.run_forever();
}

// ============================================================================

/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
///
/// We do all our USB work under interrupt, so the main thread can continue on
/// knowing nothing about USB.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    let app = APP_INSTANCE.as_mut().unwrap();
    app.usbctrl_irq();
}

// ============================================================================

// PANIC MANAGEMENT
use core::panic::PanicInfo;
#[panic_handler]
unsafe fn panic(_info: &PanicInfo) -> ! {
    let app = APP_INSTANCE.as_mut().unwrap();
    app.panic_handler();
}

// ============================================================================

// End of file
