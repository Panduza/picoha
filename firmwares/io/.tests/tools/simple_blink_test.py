import time
import json
import base64
import serial

#
ser = serial.Serial("/dev/ttyACM0", 115200)

# Set pin led as output
req = { "cod": 0, "pin": 25, "arg": 2 }
ser.write( (json.dumps(req) + "\n") .encode() )

#
while True:
    #
    req = { "cod": 1, "pin": 25, "arg": 1 }
    ser.write( (json.dumps(req) + "\n") .encode() )

    #
    time.sleep(1)

    #
    req = { "cod": 1, "pin": 25, "arg": 0 }
    ser.write( (json.dumps(req) + "\n") .encode() )

    #
    time.sleep(1)

# #
# while True:
#     line = ser.readline()
#     print(line, "\n")

# # close port
# ser.close()

