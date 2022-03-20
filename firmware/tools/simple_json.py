




import json
import base64
import serial

ser = serial.Serial("/dev/ttyACM0", 115200)

# req = { "cmd": "get_version" }
req = { "cmd": "twi_m_wr", "data": base64.b64encode(bytearray([0])).decode("utf-8"), "size": 1 }

# b64encode
ser.write( (json.dumps(req) + "\n") .encode() )

while True:
    line = ser.readline()
    print(line, "\n")


# close port
ser.close()

