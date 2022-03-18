
// ============================================================================

// USB crates
use rp_pico::hal::usb::UsbBus;
use usb_device::prelude::UsbVidPid;
use usb_device::prelude::UsbDevice;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::class_prelude::UsbBusAllocator;

// USB Communications Class Device support
use usbd_serial::SerialPort;

// ============================================================================

/// Intialize the usb device object
pub fn init_usb_serial(usb_bus: &'static UsbBusAllocator<UsbBus>) -> SerialPort<UsbBus>
{
    return SerialPort::new(&usb_bus);
}

