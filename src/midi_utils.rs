// Utility functions for MIDI messages

pub fn is_on_status(status: u8) -> bool {
    status >= 144 && status <= 159
}

pub fn is_off_status(status: u8) -> bool {
    status >= 128 && status <= 143
}

pub fn status_channel(status: u8) -> u8 {
    // Does not have a channel number
    if status >= 240 {
        return 1
    }

    status % 16 + 1
}