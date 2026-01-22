use std::io::{stdin, stdout, Write};
use crate::errors::ProgramError;
use midir::*;
use crate::return_err;

pub fn get_midi_input() -> Result<MidiInput, ProgramError> {
    // Create the MIDI input
    let mut midi_in = match MidiInput::new("midir reading input") {
        Ok(m) => m,
        Err(e) => return_err!(format!("failed to create midi input: {}", e))
    };

    midi_in.ignore(Ignore::None);

    Ok(midi_in)
}

pub fn get_midi_input_port(midi_in: &MidiInput) -> Result<MidiInputPort, ProgramError> {
    let in_ports = midi_in.ports();

    match in_ports.len() {
        0 => return_err!("no input port found"),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );

            Ok(in_ports[0].to_owned())
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
                Some(p) => Ok(p.to_owned()),
                None => return_err!("invalid input port selected")
            }
        }
    }
}

pub fn get_midi_output() -> Result<MidiOutputConnection, ProgramError> {
    let midi_out = match MidiOutput::new("Midi Output Connection") {
        Ok(m) => m,
        Err(e) => return_err!(format!("failed to create midi output: {}", e))
    };

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();

    let out_port = match out_ports.len() {
        0 => return_err!("no output port found"),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );

            Ok(out_ports[0].to_owned())
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }

            print!("Please select output port: ");
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

            match out_ports.get(parsed_input) {
                Some(p) => Ok(p.to_owned()),
                None => return_err!("invalid output port selected")
            }
        }
    }?;

    match midi_out.connect(&out_port, "midir-test") {
        Ok(c) => Ok(c),
        Err(e) => return_err!(format!("failed to connect midi output: {}", e))
    }
}
