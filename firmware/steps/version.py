import os
import pyudev
from xdocz import *
from behave import *

###############################################################################
###############################################################################

# Required to parse arguments in steps, for example "{thing}"
use_step_matcher("parse")

###############################################################################
###############################################################################

# @given('a single adapter board is connected to the test server')
# def foo_1(context):
#     # Tester responsability
#     pass

# ###############################################################################
# ###############################################################################

# @when('scanning for tty USB ids "{manufacturer}" and "{product}"')
# def foo_2(context, manufacturer, product):
#     udev_context = pyudev.Context()
#     devices = udev_context.list_devices(subsystem='tty', ID_BUS='usb', ID_VENDOR_ID=manufacturer, ID_MODEL_ID=product)
#     dev_array = [dev for dev in devices ]
#     number_of_devices = len( dev_array )
#     assert number_of_devices == 1
#     context.test_device = dev_array[0]

# ###############################################################################
# ###############################################################################

# @then('the serial number must be "{serial}"')
# def foo_3(context, serial):
#     properties = dict(context.test_device.properties)
#     assert properties["ID_SERIAL_SHORT"] == serial

