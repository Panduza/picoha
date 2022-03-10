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

def ttyPortfromUsbInfo(vendor_id, product_id, serial=None):
    context = pyudev.Context()
    for device in context.list_devices(ID_BUS='usb'):
        properties = dict(device.properties)
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
    ID_VENDOR_ID="4242"
    ID_MODEL_ID="0001"
    ID_SERIAL_SHORT="TEST_123456789"
    context.device_serial_port = ttyPortfromUsbInfo(vendor_id=ID_VENDOR_ID, product_id=ID_MODEL_ID, serial=ID_SERIAL_SHORT)
    assert context.device_serial_port is not None, f"Test adapter not connected ! {ID_VENDOR_ID}:{ID_MODEL_ID}/{ID_SERIAL_SHORT}"

###############################################################################
###############################################################################

@given('a single adapter board is connected to the test server')
def foo_1(context):
    # Tester responsability
    pass

###############################################################################
###############################################################################

@when('scanning for tty USB ids "{manufacturer}" and "{product}"')
def foo_2(context, manufacturer, product):
    udev_context = pyudev.Context()
    devices = udev_context.list_devices(ID_BUS='usb', ID_VENDOR_ID=manufacturer, ID_MODEL_ID=product)
    global __device_found
    __device_found = None
    for dev in devices:
        properties = dict(dev.properties)
        if manufacturer == properties["ID_VENDOR_ID"] and product == properties["ID_MODEL_ID"]:
            __device_found=dev
    assert __device_found is not None
    context.test_device = __device_found

###############################################################################
###############################################################################

@then('the serial number must be "{serial}"')
def foo_3(context, serial):
    properties = dict(context.test_device.properties)
    assert properties["ID_SERIAL_SHORT"] == serial

