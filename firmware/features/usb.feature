Feature: Usb

    The adapter must provide a USB CDC interface with the defined identifiers

    Scenario Outline: Find usb device
        Given a single adapter board is connected to the test server
        When scanning for tty USB ids "<manufacturer>" and "<product>"
        Then the serial number must be "<serial>"

    Examples: Usb Config
    | manufacturer      | product       | serial            |
    | 4242              | 0001          | TEST_123456789    |

