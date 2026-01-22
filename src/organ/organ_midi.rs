// This is primarily for the Harrison & Harrison Organ
// so that controlling or interpreting the stops is possible.
// This will mean that any MIDI device can control stops for the Organ,
// OR reading Sysex messages from the organ can be interpreted as the correct organ stops.

// This file will handle everything related to creating and reading SysEx messages that control those stops.
// It should translate any desired organ stop into the correct Sysex Message

/*
Organ MIDI	Messages

Here are the O&M manuals for the Organ regarding the MIDI messages and Sysex commands to control the stops.

Notes are transmitted as MIDI Note messages with velocity = 64.
Notes are received as MIDI Note messages, velocity is ignored.
Expression is transmitted/received as MIDI Master Volume.

Stops are transmitted/received as 9 byte MIDI SysEx messages.
F0-2B-01-01-22/23-nn-nn-00-F7
22 = Stop Off
23 = Stop On
nn-nn = internal stop number

In this mode stops are transmitted/received as:
F0-2B-01-01-22/23-nn-nn-F7
22 = Stop Off
23 = Stop On
nn-nn = internal stop number

 */

use crate::organ::stops_table::OrganStop;

pub fn sysex_to_organ_stop(sysex_message: &[u8]) -> Option<OrganStop> {

    // Check if on or off
    let on: bool = match sysex_message.get(4) {
        Some(22) => true,
        Some(23) => false,
        _ => return None,
    };

    // Only need the second number as the first is always 0
    match sysex_message.get(6) {
        Some(stop_number) => ,
        None => None
    }

}

pub fn organ_stop_to_sysex(organ_stop: OrganStop, on: bool) -> Vec<u8> {
    // Provide the correct sysex command for the organ stop
    // 22 if on is true, 23 if on is false

    let mut sysex_message: Vec<u8> = Vec::with_capacity(9);

    // Start of Sysex Message
    sysex_message.push(0xF0);

    // Manufacturer ID
    sysex_message.push(0x2B);

    // Device ID
    sysex_message.push(0x01);

    // Model / Function Group
    sysex_message.push(0x01);

    // Stop On/Off
    sysex_message.push(if on { 0x23 } else { 0x22 });

    // First stop number (always 0)
    sysex_message.push(0x00);

    // Second Stop Number
    // For this Viscount organ SysEx, you do not encode the stop number as little-endian.
    // You encode the MIDI stop number MSB first, then LSB (big-endian–style).
    let stop_number = organ_stop as u8;
    sysex_message.push(stop_number >> 4);

    // Padding
    sysex_message.push(stop_number & 0x0F);

    // End of Sysex Message
    sysex_message.push(0xF7);

    sysex_message
}