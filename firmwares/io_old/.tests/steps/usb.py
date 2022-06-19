import os
import pyudev
import xdocz
from behave import *

###############################################################################
###############################################################################

# Required to parse arguments in steps, for example "{thing}"
use_step_matcher("parse")

###############################################################################
###############################################################################

def ttyPortfromUsbInfo(vendor_id, product_id, serial=None, base_dev_tty="/dev/ttyACM"):
    # Explore usb device with tty subsystem
    udev_context = pyudev.Context()
    for device in udev_context.list_devices(ID_BUS='usb', SUBSYSTEM='tty'):
        properties = dict(device.properties)
        
        # Need to find the one with the DEVNAME corresponding to the /dev serial port
        if 'DEVNAME' not in properties or not properties['DEVNAME'].startswith(base_dev_tty):
            continue

        # Check vendor/product/serial
        if vendor_id == properties["ID_VENDOR_ID"] and product_id == properties["ID_MODEL_ID"]:
            if serial:
                if serial == properties["ID_SERIAL_SHORT"]:
                    return properties["DEVNAME"]
            else:
                return properties["DEVNAME"]

    return None

###############################################################################
###############################################################################

@given('the test adapter connected')
def foo_0(context):
    context.device_serial_port = ttyPortfromUsbInfo(vendor_id=context.USB_VENDOR_ID, product_id=context.USB_PRODUCT_ID, serial=context.USB_SERIAL_TEST)
    assert context.device_serial_port is not None, f"Test adapter not connected ! {context.USB_VENDOR_ID}:{context.USB_PRODUCT_ID}/{context.USB_SERIAL_TEST}"
    xdocz.AttachTextLog(context, f"serial port: {context.device_serial_port}")

###############################################################################
###############################################################################

@given('a single adapter board is connected to the test server')
def foo_1(context):
    # Tester responsability
    pass

###############################################################################
###############################################################################

@when('scanning for tty USB ids must be 0x16c0 and 0x05e1')
def foo_2(context):
    udev_context = pyudev.Context()
    devices = udev_context.list_devices(ID_BUS='usb', ID_VENDOR_ID=context.USB_VENDOR_ID, ID_MODEL_ID=context.USB_PRODUCT_ID)
    global __device_found
    __device_found = None
    for dev in devices:
        properties = dict(dev.properties)
        if context.USB_VENDOR_ID == properties["ID_VENDOR_ID"] and context.USB_PRODUCT_ID == properties["ID_MODEL_ID"]:
            __device_found=dev
    assert __device_found is not None
    context.test_device = __device_found

###############################################################################
###############################################################################

@then('the serial number must be TEST_123456789')
def foo_3(context):
    properties = dict(context.test_device.properties)
    assert properties["ID_SERIAL_SHORT"] == context.USB_SERIAL_TEST

