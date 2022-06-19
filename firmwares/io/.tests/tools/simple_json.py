




import json
import base64
import serial

ser = serial.Serial("/dev/ttyACM0", 115200)


# req = { "cod": 0, "pin": 0, "arg": 0 }
req = { "cod": 0 }

# b64encode
ser.write( (json.dumps(req) + "\n") .encode() )

while True:
    line = ser.readline()
    print(line, "\n")


# close port
ser.close()

