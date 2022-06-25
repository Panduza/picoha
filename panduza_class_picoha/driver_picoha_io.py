import time
import json
import base64
import pyudev
import serial
import threading
from loguru import logger
from panduza_platform import MetaDriverIo
from statemachine import StateMachine, State

# pip install python-statemachine

# Number of seconds befroe the state error try to re-init
ERROR_TIME_BEFROE_RETRY_S=3

# -----------------------------------------------------------------------------

def TTYPortfromUsbInfo(vendor_id, product_id, serial=None, base_dev_tty="/dev/ttyACM"):
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

# -----------------------------------------------------------------------------

class PicohaBridgeMachine(StateMachine):
    # States
    initialize = State('Initialize', initial=True)
    running = State('Running')
    error = State('Error')

    # Events
    init_fail = initialize.to(error)
    init_success = initialize.to(running)
    runtine_error = running.to(error)
    restart = error.to(initialize)

# -----------------------------------------------------------------------------

class PicohaBridge:
    """The bridge manage the communication between multiple interface and one device
    """

    ###########################################################################
    ###########################################################################
    
    def __init__(self, vendor_id, product_id, serial=None):
        self.usbid_vendor = vendor_id
        self.usbid_product = product_id
        self.usbid_serial = serial
        self.start_time = time.time()
        self.fsm = PicohaBridgeMachine()
        self.mutex = threading.Lock()

    ###########################################################################
    ###########################################################################
    
    def state_initialize(self):

        # Get serial port
        self.serial_port = TTYPortfromUsbInfo(self.usbid_vendor, self.usbid_product, self.usbid_serial, base_dev_tty="/dev/ttyACM")
        if self.serial_port is None:
            logger.error(f"adapter not connected !")
            self.fsm.init_fail()
            self.start_time = time.time()
            return

        # Try to initialize the serial object
        try:
            self.serial_obj = serial.Serial(self.serial_port, timeout=2)
        except serial.serialutil.SerialException:
            logger.error(f"serial cannot be initialized !")
            self.fsm.init_fail()
            self.start_time = time.time()
            return

        # Initialization ok !
        self.fsm.init_success()

    ###########################################################################
    ###########################################################################
    
    def state_running(self):
        
        # Communication test sequence
        try:
            req = { "cod": 10, "pin": 0, "arg": 0 }
            self.serial_obj.write( (json.dumps(req) + "\n") .encode() )
            line = self.serial_obj.readline()
            logger.debug(f"{line}")
        except serial.serialutil.SerialException as e:
            logger.error(f"adapter unreachable !")
            self.fsm.runtine_error()

    ###########################################################################
    ###########################################################################
    
    def state_error(self):
        global ERROR_TIME_BEFROE_RETRY_S
        if time.time() - self.start_time > ERROR_TIME_BEFROE_RETRY_S:
            self.fsm.restart()

    ###########################################################################
    ###########################################################################
    
    def loop(self):
        self.mutex.acquire()

        cs = self.fsm.current_state
        logger.debug(f"{cs}")
        if   cs.name == 'Initialize':
            self.state_initialize()
        elif cs.name == 'Running':
            self.state_running()
        elif cs.name == 'Error':
            self.state_error()

        self.mutex.release()
        return False

# -----------------------------------------------------------------------------

class DriverPicohaIO(MetaDriverIo):
    """Driver IO for the Picoha
    """
    
    # Managed bridges
    # { portname => PicohaBridge }
    Bridges = dict()

    ###########################################################################
    ###########################################################################
    
    def config(self):
        """FROM MetaDriver
        """
        return {
            "compatible": "picoha_io",
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
        usb_uuid = self.usbid_vendor + self.usbid_product + str(self.usbid_serial)
        logger.debug(f"usb_uuid = {usb_uuid}")

        # Init the bridge
        DriverPicohaIO.Bridges[usb_uuid] = PicohaBridge(self.usbid_vendor, self.usbid_product, self.usbid_serial)
        self.bridge = DriverPicohaIO.Bridges[usb_uuid]

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
        return self.bridge.loop()

    ###############################################################################
    ###############################################################################



