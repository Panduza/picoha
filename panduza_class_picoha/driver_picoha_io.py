import time
import json
import base64
import pyudev
import serial
from loguru import logger
from panduza_platform import MetaDriverIo
from statemachine import StateMachine, State

# pip install python-statemachine

# -----------------------------------------------------------------------------

class CallbackState(State):
    def __init__(self, name, cb, value=None, initial=False):
        super().__init__(name, value, initial)
        self.cb = cb

# -----------------------------------------------------------------------------


# class TrafficLightMachine(StateMachine):
#     green = State('Green', initial=True)
#     yellow = State('Yellow')
#     red = State('Red')

#     slowdown = green.to(yellow)
#     stop = yellow.to(red)
#     go = red.to(green)
    

# traffic_light = TrafficLightMachine()

# print(traffic_light.current_state)

# traffic_light.slowdown()

# print(traffic_light.current_state)







class DriverPicohaIO(MetaDriverIo):
    """Driver IO for the Picoha
    """

    ###########################################################################
    ###########################################################################
    
    def config(self):
        """FROM MetaDriver
        """
        return {
            "compatible": "picoha_twi_master",
            "info": { "type": "io", "version": "1.0" },
            "settings": {
                "usbid_vendor":  "[optional] Usb vendor ID in the following format (\"16c0\" : default)",
                "usbid_product": "[optional] Usb product ID in the following format (\"05e1\" : default)",
                "usbid_serial":  "[optional] Usb serial ID",
            }
        }

    ###########################################################################
    ###########################################################################

    def setup(self, tree):
        """FROM MetaDriver
        """
        # Initialize properties
        self.usbid_vendor = "16c0"
        self.usbid_product = "05e1"
        self.usbid_serial = None

        # Get serial port
        self.serial_port = self.ttyPortfromUsbInfo(self.usbid_vendor, self.usbid_product, self.usbid_serial, base_dev_tty="/dev/ttyACM")
        if self.serial_port is None:
            raise Exception(f"Serial Not Found")

        #
        self.ser = serial.Serial(self.serial_port, 115200)



        #
        logger.debug(f"interface ready")


    ###########################################################################
    ###########################################################################

    def on_start(self):
        """From MetaDriver
        """
        pass
        
    ###########################################################################
    ###########################################################################

    def loop(self):
        """FROM MetaDriver
        """
        return False

    ###############################################################################
    ###############################################################################

    def ttyPortfromUsbInfo(self, vendor_id, product_id, serial=None, base_dev_tty="/dev/ttyACM"):
        """Find tty port from usb information
        """
        # Explore usb device with tty subsystem
        udev_context = pyudev.Context()
        for device in udev_context.list_devices(ID_BUS='usb', SUBSYSTEM='tty'):
            properties = dict(device.properties)
            
            # Need to find the one with the DEVNAME corresponding to the /dev serial port
            if 'DEVNAME' not in properties or not properties['DEVNAME'].startswith(base_dev_tty):
                continue

            # Check vendor/product/serial
            if vendor_id == properties["ID_VENDOR_ID"] and product_id == properties["ID_MODEL_ID"]:
                if serial:
                    if serial == properties["ID_SERIAL_SHORT"]:
                        return properties["DEVNAME"]
                else:
                    return properties["DEVNAME"]

        return None

    ###############################################################################
    ###############################################################################


