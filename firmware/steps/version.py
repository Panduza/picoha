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
    
    req = { "cmd": "get_version" }
    ser.write(b'hello')

    line = ser.readline()
    print(line, '\n')

    # close port
    ser.close()

###############################################################################
###############################################################################

@then('it must respond with the firmware and hardware version')
def foo_3(context):
    pass

