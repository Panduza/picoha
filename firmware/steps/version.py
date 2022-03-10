import os
import serial
import pyudev
from xdocz import *
from behave import *

###############################################################################
###############################################################################

# Required to parse arguments in steps, for example "{thing}"
use_step_matcher("parse")

###############################################################################
###############################################################################

@when('the host requests the version to the adapter')
def foo_2(context):

    # open serial port
    ser = serial.Serial(context.device_serial_port, 115200)
    # >>> print(ser.name)         # check which port was really used
    # >>> ser.write(b'hello')     # write a string
    
    # close port
    ser.close()

###############################################################################
###############################################################################

# @then('the serial number must be "{serial}"')
# def foo_3(context, serial):
#     properties = dict(context.test_device.properties)
#     assert properties["ID_SERIAL_SHORT"] == serial

