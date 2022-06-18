Feature: Version

    The adapter must be able to provides its firmware and hardware version

    Scenario: Request the version
        Given the test adapter connected
        When the host requests the version to the adapter
        Then it must respond with the firmware and hardware version



