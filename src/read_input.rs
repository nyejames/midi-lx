use std::fmt::format;
use std::io::{stdin, stdout, Write};
use crate::{return_err, ProgramError};
use midir::*;

pub fn run() -> Result<(), ProgramError> {
    let mut input = String::new();

    let mut midi_in = match MidiInput::new("midir reading input") {
        Ok(m) => m,
        Err(e) => return_err!(format!("failed to create midi input: {}", e))
    };

    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple is available)
    let in_ports = midi_in.ports();

    let in_port = match in_ports.len() {
        0 => return_err!("no input port found"),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );

            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");

            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }

            print!("Please select input port: ");

            match stdout().flush() {
                Ok(_) => (),
                Err(e) => return_err!(format!("failed to flush stdout: {e}")),
            }

            let mut input = String::new();
            match stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(e) => return_err!(format!("failed to read line from stdin: {e}")),
            }

            let parsed_input = match input.trim().parse::<usize>() {
                Ok(i) => i,
                Err(e) => {
                    return_err!(
                        format!("failed to parse input. Expected a number, got '{e}' instead")
                    )
                }
            };

            match in_ports.get(parsed_input) {
                Some(p) => p,
                None => return_err!("invalid input port selected")
            }
        }
    };

    println!("\nOpening connection");
    let in_port_name = match midi_in.port_name(in_port) {
        Ok(n) => n,
        Err(e) => return_err!(format!("failed to get port name: {}", e))
    };

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = match midi_in.connect(
        in_port,
        "midir-read-input",
        move |stamp, message, _| {
            println!("{}: {:?} (len = {})", stamp, message, message.len());
        },
        (),
    ) {
        Ok(connection) => connection,
        Err(e) => return_err!(&format!("failed to connect: {}", e))
    };

    println!(
        "Connection open, reading input from '{}' (press any key to exit) ...",
        in_port_name
    );

    input.clear();

    // Wait for the next key press
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => return_err!(&format!("failed to read line from stdin: {e}")),
    }

    println!("Closing connection");
    Ok(())
}