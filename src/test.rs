use std::io::stdin;
use std::thread::sleep;
use std::time::Duration;
use crate::{return_err, ProgramError};
use crate::midi_io::{get_midi_input, get_midi_input_port, get_midi_output};
use crate::midi_translator::translate_midi_to_chamsys_command;

pub fn dummy_midi_out() -> Result<(), ProgramError> {
    println!("\nRUNNING TEST PROGRAM");
    let mut conn_out = get_midi_output()?;
    let midi_in = get_midi_input()?;
    let in_port = get_midi_input_port(&midi_in)?;

    let in_port_name = match midi_in.port_name(&in_port) {
        Ok(n) => n,
        Err(e) => return_err!(format!("failed to get port name: {}", e))
    };

    println!(
        "Connection open, reading input from '{}' (press any key to exit) ...",
        in_port_name
    );

    let _conn_in = match midi_in.connect(
        &in_port,
        "midir-read-input",
        move |stamp, message, _| {

            // RUN ANY INPUT TESTS IN HERE
            let command = translate_midi_to_chamsys_command(message);

            println!("MIDI {}: {:?}", stamp, message);

            match command {
                Ok(c) => {
                    if let Some(c) = c {
                        println!("COMMAND: {}", c);
                    } else {
                        println!("No command set for this MIDI input");
                    }
                },
                Err(e) => {
                    println!("Error translating MIDI to command: {}", e)
                },
            }
        },
        (),
    ) {
        Ok(connection) => connection,
        Err(e) => return_err!(&format!("failed to connect: {}", e))
    };

    // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
    let mut play_note = |note: u8, duration: u64| {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x64;

        // We're ignoring errors in here
        let _ = conn_out.send(&[NOTE_ON_MSG, note, VELOCITY]);
        sleep(Duration::from_millis(duration * 150));
        let _ = conn_out.send(&[NOTE_OFF_MSG, note, VELOCITY]);
    };

    sleep(Duration::from_millis(4 * 150));

    play_note(66, 4);
    play_note(65, 3);
    play_note(63, 1);
    play_note(61, 6);
    play_note(59, 2);
    play_note(58, 4);
    play_note(56, 4);
    play_note(54, 4);

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => return_err!(format!("failed to read line from stdin: {e}")),
    }

    println!("\nDONE playing some notes LOL");

    Ok(())
}