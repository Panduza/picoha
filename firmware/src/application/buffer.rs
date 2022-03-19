use rp_pico::hal;
use rp_pico::hal::pac;

pub struct BufferError;

// ============================================================================

/// Buffer to manage usb data (safe with usb interrupts)
pub struct UsbBuffer<const CAPACITY: usize> {
    // atomic bool mutex
    ///
    buffer: [u8; CAPACITY],

    /// current number of data loaded
    size: usize,
}

// ============================================================================

///
impl<const CAPACITY: usize> UsbBuffer<CAPACITY> {
    ///
    pub fn new() -> Self {
        Self {
            buffer: [0; CAPACITY],
            size: 0,
        }
    }

    /// Load the buffer from usb serial
    pub fn load(&mut self, src: &[u8], size: usize) {
        if self.size + size < CAPACITY {
            self.buffer[self.size..].copy_from_slice(&src[0..size]);
            self.size += size;
        }
    }

    ///
    pub fn get_command(&mut self) -> Option<[u8; 512]> {
        // Disable the USB interrupt
        pac::NVIC::mask(hal::pac::Interrupt::USBCTRL_IRQ);

        // Init command buffer
        let mut cmd: Option<[u8; 512]> = None;

        // Check for a complete command (end with \n or \r)
        match self.buffer[0..self.size]
            .iter()
            .position(|&c| c == '\r' as u8 || c == '\n' as u8)
        {
            None => {
                // Do nothing
            }
            Some(index) => {
                let position: usize = index as usize;
                let mut cmd_buf = [0u8; 512];
                cmd_buf[0..position].copy_from_slice(&self.buffer[0..position]);
                cmd = Some(cmd_buf);
                self.buffer.rotate_left(position);
            }
        }

        // Enable the USB interrupt
        unsafe {
            pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
        }

        cmd
    }
}

// ============================================================================
