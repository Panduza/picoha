# pip install pyusb

# import sys
# import usb.core

# devs = usb.core.find(find_all=True)
# for cfg in devs:
#     print( "SERIAL NUM : " , usb.util.get_string( cfg, cfg.iSerialNumber ) )
#     sys.stdout.write('Decimal VendorID=' + str(cfg.idVendor) + ' & ProductID=' + str(cfg.idProduct) + '\n')
#     sys.stdout.write('Hexadecimal VendorID=' + hex(cfg.idVendor) + ' & ProductID=' + hex(cfg.idProduct) + '\n\n')
    

# dev = usb.core.find(idVendor=0x16c0, idProduct=0x27dd)
# # dev = usb.core.find(idVendor=0x2e8a, idProduct=0x0003)
# if dev is None:
#     raise ValueError('Our device is not connected')


# print( usb.util.get_string( dev, dev.iSerialNumber ) )





import pyudev



def getSerialFor(vendor_id, product_id):
    context = pyudev.Context()
    for device in context.list_devices(ID_VENDOR_ID=vendor_id, ID_MODEL_ID=product_id):
        properties = dict(device.properties)
        return properties['ID_SERIAL_SHORT']
    return None

def ttyPortfromUsbInfo(vendor_id, product_id, serial=None):
    context = pyudev.Context()
    for device in context.list_devices(subsystem='tty', ID_BUS='usb'):
        properties = dict(device.properties)
        if vendor_id == properties["ID_VENDOR_ID"] and product_id == properties["ID_MODEL_ID"]:
            if serial:
                if serial == properties["ID_SERIAL_SHORT"]:
                    return properties["DEVNAME"]    
            else:
                return properties["DEVNAME"]
    return None


# print(getSerialFor(vendor_id='16c0', product_id='27dd'))
# print(getSerialFor(vendor_id='16c0', product_id='27dd'))
# print( ttyPortfromUsbInfo(vendor_id='16c0', product_id='27dd', serial='123456789')) 

