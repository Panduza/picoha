
use super::HostAdapter;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;


///
impl<OP> HostAdapter<OP> where OP: OutputPin {

    /// Panic handler implementation for the application
    pub fn panic_handler(&mut self) -> ! {
        loop {

            // self.usb_serial.write(b"PANIC").ok();

            self.led_pin.set_high().ok();
            self.delay.delay_ms(100);
            self.led_pin.set_low().ok();
            self.delay.delay_ms(100);
        }
    }

}
