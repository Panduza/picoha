use rp_pico::hal;
use rp_pico::hal::pac;

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
    pub fn load(&mut self, src: &[u8], count: usize) {
        if self.size + count < CAPACITY {
            self.buffer[self.size..self.size + count].copy_from_slice(&src[0..count]);
            self.size += count;
        }
    }

    ///
    pub fn get_command(&mut self, dest: &mut [u8; CAPACITY]) -> Option<usize> {
        // Disable the USB interrupt
        pac::NVIC::mask(hal::pac::Interrupt::USBCTRL_IRQ);

        // Init command buffer
        let mut cmd: Option<usize> = None;

        // Check for a complete command (end with \n or \r)
        match self.buffer[0..self.size]
            .iter()
            .position(|&c| c == '\r' as u8 || c == '\n' as u8)
        {
            None => {
                // Do nothing
            }
            Some(index) => {
                // Position is the index of the \r
                let position: usize = index as usize;
                // count is the size of the command
                let count = position + 1;
                //
                // let mut cmd_buf = [0u8; CAPACITY];
                dest[0..position].copy_from_slice(&self.buffer[0..position]);
                cmd = Some(position);
                self.buffer.rotate_left(count);
                self.size -= count;
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
