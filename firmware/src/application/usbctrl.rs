
use super::HostAdapter;

use embedded_hal::digital::v2::OutputPin;

// ============================================================================

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


///
impl<OP> HostAdapter<OP> where OP: OutputPin {



    pub fn usbctrl_irq(&mut self) {

        // Poll the USB driver with all of our supported USB Classes
        if self.usb_device.poll(&mut [self.usb_serial]) {
            
            // Buffer to read the serial port
            let mut serial_buffer = [0u8; 512];
            match self.usb_serial.read(&mut serial_buffer) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(0) => {
                    // Do nothing
                }
                Ok(count) => {

                    self.usb_buffer.load(&serial_buffer, count);



                    for i in 0..count {
                        let c = serial_buffer[i];

                        // let mut bbbb = [0u8; 4];
                        // let oo = base_10_bytes(c, &mut bbbb);

                        // let mut bbbb2 = [0u8; 4];
                        // let oo8 = base_10_bytes('\n' as u8, &mut bbbb2);

                        // self.usb_serial.write(&[c]).unwrap();
                        // self.usb_serial.write(b" ").unwrap();
                        // self.usb_serial.write(&oo).unwrap();
                        // self.usb_serial.write(b" ").unwrap();
                        // self.usb_serial.write(&oo8).unwrap();
                        // self.usb_serial.write(b" \r\n").unwrap();

                        if c == 'z' as u8 {
                            let _ = self.usb_serial.write(b"{ \"debug\": \"Hello!\" }\r\n");
                        }

                        // let mm = b"{ \"debug\": \"Hello return\" }\r\n";
                        // for n in 0..mm.len() {
                        //     CmdBuffer[n] = mm[n];
                        // }

                        if c == '\r' as u8 || c == '\n' as u8 {
                            let _ = self.usb_serial.write(b"{ \"debug\": \"Hello return\" }\r\n");

                            // producer.enqueue(CmdBuffer).ok().unwrap();
                        }
                    }
                }
            }
        }
    }
}

