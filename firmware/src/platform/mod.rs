// ============================================================================

// USB crates
use rp_pico::hal::usb::UsbBus;
use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::UsbDevice;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;

// USB Communications Class Device support
use usbd_serial::SerialPort;

// ============================================================================

mod config;

// ============================================================================

/// Create a USB device with a fake VID and PID
pub fn init_usb_device(usb_bus: &'static UsbBusAllocator<UsbBus>) -> UsbDevice<UsbBus> {
    UsbDeviceBuilder::new(
        &usb_bus,
        UsbVidPid(config::USB_MANUFACTURER_ID, config::USB_PRODUCT_ID),
    )
    .manufacturer(config::USB_MANUFACTURER_NAME)
    .product(config::USB_PRODUCT_NAME)
    .serial_number(config::USB_SERIAL_NUMBER)
    .device_class(2) // from: https://www.usb.org/defined-class-codes
    .build()
}

// ============================================================================

/// Intialize the usb device object
pub fn init_usb_serial(usb_bus: &'static UsbBusAllocator<UsbBus>) -> SerialPort<UsbBus> {
    return SerialPort::new(&usb_bus);
}

// ============================================================================
