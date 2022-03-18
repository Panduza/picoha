// ============================================================================

// HAL
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal;
use rp_pico::hal::gpio;

// USB crates
use rp_pico::hal::usb::UsbBus;
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::UsbDevice;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;

// USB Communications Class Device support
use usbd_serial::SerialPort;

// ============================================================================

mod buffer;
use buffer::CommandBuffer;

// ============================================================================

/// Store all the usefull objects for the application
pub struct HostAdapter<OP> where OP: OutputPin {
    /// To manage delay
    delay: cortex_m::delay::Delay,

    /// Led pin control
    led_pin: OP,

    /// The USB Device Driver (shared with the interrupt).
    usb_device: &'static mut UsbDevice<'static, hal::usb::UsbBus>,

    /// The USB Serial Device Driver (shared with the interrupt).
    usb_serial: &'static mut SerialPort<'static, hal::usb::UsbBus>,

    ///
    cmd_buffer: CommandBuffer
}

// ============================================================================

/// Implementation of the App
impl<OP> HostAdapter<OP> where OP: OutputPin {
    /// Application intialization
    pub fn new(
        delay: cortex_m::delay::Delay,
        led_pin: OP,
        usb_dev: &'static mut UsbDevice<'static, hal::usb::UsbBus>,
        usb_ser: &'static mut SerialPort<'static, hal::usb::UsbBus>,
    ) -> Self {
        Self {
            delay: delay,
            led_pin: led_pin,
            usb_device: usb_dev,
            usb_serial: usb_ser,
            cmd_buffer: CommandBuffer::new()
        }
    }

    /// Main loop of the main task of the application
    pub fn run_forever(&mut self) -> ! {
        let obj = self;
        loop {
            obj.led_pin.set_high().ok();
            obj.delay.delay_ms(500);
            obj.led_pin.set_low().ok();
            obj.delay.delay_ms(500);
        }
    }


}

// Method Implementations
mod panic;
mod usbctrl;
