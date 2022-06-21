import time
import json
import base64
import pyudev
import serial
from loguru import logger
from pza_platform import MetaDriver

class DriverPicohaTwiMaster(MetaDriver):
    """ Driver twi Master
    """

    ###########################################################################
    ###########################################################################
    
    def config(self):
        """ FROM MetaDriver
        """
        return {
            "compatible": "picoha_twi_master",
            "info": { "type": "twi/master", "version": "1.0" },
            "settings": {
                "usbid_vendor": "[optional] Usb vendor ID in the following format (\"16c0\" : default)",
                "usbid_product": "[optional] Usb product ID in the following format (\"05e1\" : default)",
                "usbid_serial": "[optional] Usb serial ID",
            }
        }

        # "bitrate_hz": "spi bitrate as an integer in hz (4000000)",
        # "clock_polarity": "CPOL [0 / 1]",
        # "clock_phase": "CPHA [0 / 1]",
        # "bitorder": "[msb / lsb] first",
        # "ss_polarity": "[active_low / active_high]"

    ###########################################################################
    ###########################################################################

    def setup(self, tree):
        """ FROM MetaDriver
        """
        # Initialize properties
        self.usbid_vendor = "16c0"
        self.usbid_product = "05e1"
        self.usbid_serial = None

        # Get serial port
        self.serial_port = self.ttyPortfromUsbInfo(self.usbid_vendor, self.usbid_product, self.usbid_serial, base_dev_tty="/dev/ttyACM")
        if self.serial_port is None:
            raise Exception(f"Serial Not Found")

        # Serial found continue
        logger.debug(f"serial port: {self.serial_port}")

        # Register commands
        # self.register_command("data/read", self.__data_read)
        # self.register_command("data/write", self.__data_write)
        self.register_command("data/writeRead", self.__data_write_read)

        #
        self.ser = serial.Serial(self.serial_port, 115200)

        # # Get bitrate
        # self.bitrate_khz = 1000
        # if "bitrate_hz" in tree["settings"]:
        #     self.bitrate_khz = int(tree["settings"]["bitrate_hz"] / 1000)

        # #
        # logger.debug(f"bitrate: {self.bitrate_khz}khz")

        # #
        # AardvarkBridge.ConfigureTwiMaster(self.aa_handle, self.bitrate_khz)

        #
        logger.debug(f"interface ready")


    ###########################################################################
    ###########################################################################

    def on_start(self):
        """ From MetaDriver
        """
        pass
        
    ###########################################################################
    ###########################################################################

    def loop(self):
        """ FROM MetaDriver
        """
        return False

    ###########################################################################
    ###########################################################################

    def __data_write(self, payload):
        """
        """
        # Debug log
        logger.debug(f"CMD data/write ({payload})")

        # # Parse the params
        # req = self.payload_to_dict(payload)
        # data = base64.b64decode(req["data"])
        # addr = req["addr"]
        
        # flags = AA_I2C_NO_FLAGS
        # if "addr_10b" in req and req["addr_10b"]:
        #     flags = flags | AA_I2C_10_BIT_ADDR
        # if "no_stop" in req and req["no_stop"]:
        #     flags = flags | AA_I2C_NO_STOP

        # #
        # status = aa_i2c_write(self.aa_handle, addr, flags, array('B', data) )
        # if status < 0:
        #     print(f"fail sending data ({aa_status_string(status)})")
        #     return
    
        # logger.debug(f"CMD data/read ({payload})")


    ###########################################################################
    ###########################################################################

    # def __data_read(self, payload):
    #     """
    #     """
    #     # Debug log
    #     logger.debug(f"CMD data/read ({payload})")


    ###########################################################################
    ###########################################################################

    def __data_write_read(self, payload):
        """
        """
        # Debug log
        logger.debug(f"CMD data/writeRead ({payload})")

        # Parse the params
        req = self.payload_to_dict(payload)
        data = base64.b64decode(req["data"])
        addr = req["addr"]
        
        #
        srequest = {
            "cmd": "twi_rw"
        }

        #
        srequest_str = json.dumps(srequest) + '\r\n'
        self.ser.write(srequest_str.encode())

        #
        line = self.ser.readline()
        print(line, "\n")

    ###############################################################################
    ###############################################################################

    def ttyPortfromUsbInfo(self, vendor_id, product_id, serial=None, base_dev_tty="/dev/ttyACM"):
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


