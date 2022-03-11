Feature: Usb

    The adapter must provide a USB CDC interface with the defined identifiers

    Scenario: Find usb device
        Given a single adapter board is connected to the test server
        When scanning for tty USB ids must be 0x16c0 and 0x05e1
        Then the serial number must be TEST_123456789

