use crate::errors::{ProgramError};
use crate::return_err;

pub fn translate_midi_to_chamsys_command(message: &[u8], previous_playback: &mut u8) -> Result<Option<String>, ProgramError> {
    // MIDI note number for the first note that will control PB1 on the desk
    let first_playback_note = 48;

    // Get the note value of the message
    let note = match message.get(1) {
        Some(n) => *n,
        None => return_err!("message does not contain a note value")
    };

    let status = match message.get(0) {
        Some(s) => *s,
        None => return_err!("message does not contain a status byte")
    };

    let velocity = match message.get(2) {
        Some(v) => *v,
        None => return_err!("message does not contain a velocity value")
    };

    // Important Status messages
    println!("MIDI input: {:?}", message);

    let command_letter: &str;

    // Note on for all channels
    if status >= 144 && status <= 159  {
        command_letter = "A";

    // Note off for all channels
    } else if status >= 128 && status <= 143 {
        command_letter = "R";

    // MOD WHEEL (LOL)
    } else if status == 176 {
        return Ok(Some(format!("{},{}L", previous_playback, velocity)))
    } else {
        // If this status isn't set as a command yet
        println!("Status {} not set as a command", status);
        return Ok(None);
    }

    // If is just regular note status, fill in playback info and command

    // To make sure it doesn't try to use negative playback numbers (u8 overflow panic)
    if note < first_playback_note { return Ok(None) }

    // Convert the note value to a Chamsys playback
    let playback_number = (note - first_playback_note + 1);

    // Update which playback is currently playing
    *previous_playback = playback_number;

    // Send back the formatted command
    Ok(Some(format!("{}{}", playback_number, command_letter)))
}