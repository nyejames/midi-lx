use std::io::stdin;
use crate::errors::ProgramError;
use crate::midi_io::{get_midi_input, get_midi_input_port, get_midi_output};
use crate::return_err;

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

            // Print what the MIDI in data is
            println!("MIDI {}: {:?}", stamp, message);

        },
        (),
    ) {
        Ok(connection) => connection,
        Err(e) => return_err!(&format!("failed to connect: {}", e))
    };

    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => return_err!(format!("failed to read line from stdin: {e}")),
    }

    println!("\nDONE playing some notes LOL");

    Ok(())
}