use crate::{return_err, ProgramError};

pub fn translate_note_to_sysex(message: Vec<u8>) -> Result<Vec<u8>, ProgramError> {
    // Translate the MIDI note to a corresponding sysex message

    // This will allow any MIDI signal to control any SysEx message
    // This can be used for controlling the stops on the organ with any note or other MIDI controller

    // Get the note value of the message
    let note_value = match message.get(1) {
        Some(n) => *n,
        None => return_err!("message does not contain a note value")
    };

    Ok(Vec::new())
}


pub fn translate_midi_to_chamsys_command(message: &[u8]) -> Result<&str, ProgramError> {
    // Get the note value of the message
    let note_value = match message.get(1) {
        Some(n) => *n,
        None => return_err!("message does not contain a note value")
    };

    // Convert the note value to a Chamsys playback
    match note_value {
        48 => Ok("1A"),
        49 => Ok("2A"),
        50 => Ok("3A"),
        _ => Ok("No command set for this MIDI input")
    }
}