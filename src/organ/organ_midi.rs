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
use std::io::stdin;
use color_print::cprintln;
use crate::errors::ProgramError;
use crate::midi_io::{get_midi_input, get_midi_input_port, get_midi_output};
use crate::midi_utils::{is_on_status, status_channel};
use crate::organ::stops_table::{OrganStop, TOTAL_STOPS};
use crate::return_err;

// Some test bindings of MIDI notes
pub fn play_organ(control_stops: bool) -> Result<(), ProgramError> {
    cprintln!("\n<green>RUNNING ORGAN MIDI CONTROL</>");

    let mut conn_out = get_midi_output()?;
    let midi_in = get_midi_input()?;
    let in_port = get_midi_input_port(&midi_in)?;

    let _conn_in = match midi_in.connect(
        &in_port,
        "midir-read-input",
        move |stamp, message, _| {
            let midi_message = midi_to_organ_note(message, control_stops);

            println!("MIDI message received: {:?}", message);

            if message != midi_message {
                println!("MIDI message converted: {:?}", midi_message);
            }

            // Pass this midi message through to the output
            let _ = conn_out.send(&midi_message);
        },
        (),
    ) {
        Ok(connection) => connection,
        Err(e) => return_err!(&format!("failed to connect: {}", e))
    };

    // Just wait for the user to press a key, other than the exit button to exit
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => return_err!(format!("failed to read line from stdin: {e}")),
    }

    Ok(())
}

// Converts the MIDI note to an organ MIDI note
// This translates certain MIDI info to Sysex Commands for stops,
// And converts velocity data to expression data instead (organ ignored velocity)
const TEMP_ORGAN_STOP_CHANNEL: u8 = 2;

pub fn midi_to_organ_note(message: &[u8], control_stops: bool) -> Vec<u8> {
    let mut converted_midi_message = message.to_vec();

    let note_number = match message.get(1) {
        Some(note) => *note,
        None => return converted_midi_message,
    };

    let status = match message.get(0) {
        Some(status) => *status,
        None => return converted_midi_message,
    };

    // For testing, we will arbitrarily turn certain notes into Sysex stop commands
    // Later, the user will configure these in the UI
    // We are just going to directly use the MIDI note number as the stop number
    // Lowest note on the keyboard is usually 0, so the stop numbers will start from there
    // If control_stops is false, then a normal midi note will be played from the keyboard instead.

    // Return a note converted to an organ Sysex message
    if note_number < TOTAL_STOPS && control_stops {
        // Convert the MIDI note to an organ stop number
        let stop_number = match OrganStop::from_u8(note_number) {
            Some(stop) => stop,
            None => return converted_midi_message,
        };

        let on = is_on_status(status);

        return organ_stop_to_sysex(stop_number, on)
    }

    // Otherwise just pass the MIDI data through
    // Could do weird stuff like convert velocity to expression data for the organ,
    // but I'm sure organists would hate this.
    converted_midi_message
}

pub fn sysex_to_organ_stop(sysex_message: &[u8]) -> Option<OrganStop> {

    // Check if on or off
    let on: bool = match sysex_message.get(4) {
        Some(22) => true,
        Some(23) => false,
        _ => return None,
    };

    // Only need the second number as the first is always 0
    match sysex_message.get(6) {
        Some(stop_number) => OrganStop::from_u8(*stop_number),
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