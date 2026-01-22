// This file will handle the CLI commands.
// It will eventually be replaced with a GUI but still be used for testing and development.
use crate::test::{dummy_midi_out};
use crate::chamsys::midi_through_to_chamsys;

enum UserCommand {
    Invalid, // Not a valid command
    MIDITest,
    ChamsysMIDI,
    OrganSysex,
}

pub fn run_cli() {
    let mut command = UserCommand::Invalid;

    // Get user command from CLI and run that program
    // Display errors to the user if there is a problem,
    // then either relaunch the program or prompt the user if they want to run a different program


    // Match the prompt and run the appropriate program
    match command {
        UserCommand::MIDITest => {
            println!("Running MIDI test...");
            // Currently just running the dummy program for testing
            match dummy_midi_out() {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            };
        },

        UserCommand::ChamsysMIDI => {
            // Currently just running the dummy program for testing
            match midi_through_to_chamsys() {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            };
        }

        UserCommand::OrganSysex => {
            println!("OrganSysex")
        },

        _ => {
            println!("Invalid command")
        }
    }
}