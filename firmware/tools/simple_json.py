




import json
import serial

ser = serial.Serial("/dev/ttyACM0", 115200)

req = { "cmd": "get_version" }
ser.write( (json.dumps(req) + "\n") .encode() )

while True:
    line = ser.readline()
    print(line, "\n")


# close port
ser.close()

