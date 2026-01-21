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


pub fn translate_midi_to_chamsys_command(message: &[u8]) -> Result<Option<String>, ProgramError> {
    let mut command: Option<String> = None;

    // MIDI note number for the first note that will control PB1 on the desk
    let first_playback_note = 48;

    // Get the note value of the message
    let note_value = match message.get(1) {
        Some(n) => *n,
        None => return_err!("message does not contain a note value")
    };

    let status = match message.get(0) {
        Some(s) => *s,
        None => return_err!("message does not contain a status byte")
    };

    // Important Status messages
    println!("MIDI input: {:?}", message);

    // To make sure it doesn't try to use negative playback numbers (u8 overflow panic)
    if note_value < first_playback_note { return Ok(None) }

    // Note on for all channels
    if status >= 144 && status <= 159  {
        // Convert the note value to a Chamsys playback
        let playback_number = (note_value - first_playback_note + 1) % 12;

        // Start the playback on Note On
        command = Some(format!("{}T", playback_number));
    }

    // Note off for all channels
    if status >= 128 && status <= 143  {
        // Convert the note value to a Chamsys playback
        let playback_number = (note_value - first_playback_note + 1) % 12;

        // Stop the playback on Note Off
        command = Some(format!("{}U", playback_number));
    }

    // Command will be none by default if not set
    Ok(command)
}