use std::env;
use std::net::Ipv4Addr;
use color_print::{ceprintln, cprintln};
use crate::midi_io::{get_midi_input, get_midi_input_port};
use crate::test::{dummy_midi_out};
use crate::MidiRuntime;
use crate::organ::organ_midi::play_organ;

enum Command {
    MIDITest,
    ChamsysMIDI,
    OrganStopControl,
    OrganKeyboardControl,
    Help,
}

// This function will handle the CLI commands.
// It will eventually be replaced with a GUI but still be used for testing and development.
pub fn run_cli() {
    // Get user command from CLI and run that program
    // Display errors to the user if there is a problem,
    // then either relaunch the program or prompt the user if they want to run a different program
    let args: Vec<String> = env::args().collect();

    let command = match get_command(&args[1..]) {
        Ok(command) => command,
        Err(e) => {
            ceprintln!("<red>{}</>\n", e);
            print_possible_commands();
            return;
        }
    };

    // Match the prompt and run the appropriate program
    match command {
        Command::Help => print_possible_commands(),

        Command::MIDITest => {
            println!("Running MIDI test...");
            // Currently just running the dummy program for testing
            match dummy_midi_out() {
                Ok(_) => (),
                Err(e) => {
                    ceprintln!("<red>{}</>", e)
                },
            };
        },

        // Currently just running the dummy program for testing
        Command::ChamsysMIDI => {
            let midi_input = match get_midi_input() {
                Ok(i) => i,
                Err(e) => {
                    ceprintln!("<red>{}</>", e);
                    return
                },
            };

            let selected_midi_port = match get_midi_input_port(&midi_input) {
                Ok(p) => p,
                Err(e) => {
                    ceprintln!("<red>{}</>", e);
                    return
                },
            };

            match MidiRuntime::create(
                // TEMP DEFAULTS FOR TESTING
                Ipv4Addr::new(2, 0, 0, 35),
                Ipv4Addr::new(2, 0, 0, 1),
                midi_input,
                selected_midi_port,
                None,
            ) {
                Ok(_) => (),
                Err(e) => {
                    ceprintln!("<red>{}</>", e)
                },
            };
        }

        Command::OrganStopControl => {
            match play_organ(true) {
                Ok(_) => (),
                Err(e) => {
                    ceprintln!("<red>{}</>", e)
                },
            }
        },

        Command::OrganKeyboardControl => {
            match play_organ(false) {
                Ok(_) => (),
                Err(e) => {
                    ceprintln!("<red>{}</>", e)
                },
            }
        },
    }
}

fn get_command(args: &[String]) -> Result<Command, String> {
    let command = args.first().map(String::as_str);

    match command {
        Some("lx") => Ok(Command::ChamsysMIDI),
        Some("test") => Ok(Command::MIDITest),
        Some("organ") => Ok(Command::OrganKeyboardControl),
        Some("stops") => Ok(Command::OrganStopControl),

        // Nothing entered into the command line
        None => Ok(Command::Help),

        // Some unknown command
        _ => Err(format!("Invalid command: '{}'", command.unwrap())),
    }
}

fn print_possible_commands() {
    cprintln!("\n<yellow, bold>Possible commands</>");
    cprintln!("<bold>test</> - Run the MIDI test program");
    cprintln!("<bold>lx</> - Run the Chamsys MIDI through program");
    cprintln!("<bold>organ</> - Run the organ MIDI control program");
    cprintln!("<bold>stops</> - Run the organ MIDI control program");
}