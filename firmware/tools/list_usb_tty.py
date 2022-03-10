import pyudev

print("Scan udev for usb tty devices")
context = pyudev.Context()
for device in context.list_devices(ID_BUS='usb'):
    properties = dict(device.properties)
    print("-------------------------")
    if 'ID_BUS' in properties:
        print(f" - ID_BUS:              {properties['ID_BUS']}")    
    if 'SUBSYSTEM' in properties:
        print(f" - SUBSYSTEM:           {properties['SUBSYSTEM']}")    
    if 'ID_VENDOR_ID' in properties:
        print(f" - ID_VENDOR_ID:        {properties['ID_VENDOR_ID']}")
    if 'ID_MODEL_ID' in properties:
        print(f" - ID_MODEL_ID:         {properties['ID_MODEL_ID']}")
    if 'ID_SERIAL_SHORT' in properties:
        print(f" - ID_SERIAL_SHORT:     {properties['ID_SERIAL_SHORT']}")
        
    # print(properties)

    