

use rp_pico::hal;
use rp_pico::hal::pac;


// ============================================================================

/// Buffer to manage usb data (safe with usb interrupts)
pub struct UsbBuffer<const CAPACITY: usize>
{
    buffer: [u8; CAPACITY]


    // atomic bool mutex
    // size // current number of u8 loaded


    // 
    // buf222[2..count].copy_from_slice(&buf[0..count]);

    // buf222.rotate_left(count);

}

// ============================================================================

///
impl<const CAPACITY: usize> UsbBuffer<CAPACITY> {
    ///
    pub fn new() -> Self {
        Self {
            buffer: [0; CAPACITY]
        }
    }

    pub fn load() {
        // mutex
        // While bool is true
        
        // Disable the USB interrupt
        pac::NVIC::mask(hal::pac::Interrupt::USBCTRL_IRQ);


        // Enable the USB interrupt
        unsafe{ pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ); }
    }
}



// ============================================================================

