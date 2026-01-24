use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use color_print::{ceprintln, cprintln};
use crate::errors::ProgramError;
use crate::midi_utils::{is_off_status, is_on_status};
use crate::{return_err, LxCommand, MidiRuntime};
use std::sync::mpsc;
use midir::{MidiInput, MidiInputPort, MidiOutputConnection};

/// Default port MagicQ listens on for remote control UDP
const CHAMSYS_PORT: u16 = 6553;

pub struct AppState {
    desk_ip: Ipv4Addr,

    // Should not change while running the runtime (hopefully)
    app_ip: Ipv4Addr,

    mappings: HashMap<usize, LxCommand>,
    previous_playback: u8,
}

impl AppState {
    pub fn new(desk_ip: Ipv4Addr, app_ip: Ipv4Addr) -> Self {
        Self {
            desk_ip,
            app_ip,
            mappings: HashMap::new(),
            previous_playback: 0,
        }
    }
}

pub enum AppEvent {
    Midi(Vec<u8>),
    UpdateMappings(HashMap<usize, LxCommand>),
    SetDeskIp(Ipv4Addr),
    Stop,
}

pub fn start_midi_to_chamsys_runtime(state: AppState, midi_input: MidiInput, selected_midi_port: MidiInputPort, midi_through: Option<MidiOutputConnection>) -> Result<MidiRuntime, ProgramError> {
    cprintln!("\n<green>RUNNING CHAMSYS MIDI CONTROL</>");
    let (tx, rx) = mpsc::channel::<AppEvent>();

    // MIDI prep
    let tx_midi = tx.clone();

    println!("Local IP for sending: {}", state.app_ip);

    // Try pinging the desk
    if let Ok(_) = std::process::Command::new("ping")
        .arg(state.desk_ip.to_string())
        .output() {
        println!("Ping to 2.0.0.35 succeeded");
    } else {
        println!("Ping failed — network config may be wrong");
    }

    // MIDI INPUTS MESSAGE PASSING
    let _conn_in = match midi_input.connect(
        &selected_midi_port,
        "midir-read-input",
        move |_stamp, message, _| {
            let _ = tx_midi.send(AppEvent::Midi(message.to_vec()));
        },
        (),
    ) {
        Ok(connection) => connection,
        Err(e) => return_err!(&format!("failed to connect: {}", e))
    };

    // Spawn the event loop
    std::thread::spawn(move || {
        run_event_loop(state, rx);
    });

    Ok(MidiRuntime { tx })
}

fn run_event_loop(
    mut state: AppState,
    rx: mpsc::Receiver<AppEvent>,
) {
    let socket = match UdpSocket::bind((state.app_ip, 0)) {
        Ok(s) => s,
        Err(e) => {
            ceprintln!("<red>Failed to Bind Socket: {}</>", e);
            return
        }
    };

    loop {
        match rx.recv() {
            Ok(AppEvent::Midi(message)) => {
                if let Ok(Some(cmd)) =
                    translate_midi_to_chamsys_command(
                        &message,
                        &mut state,
                    )
                {
                    match send_magicq_command(&socket, &cmd, &state) {
                        Ok(details) => println!("{}", details),
                        Err(e) => println!("{}", e)
                    }
                }
            }

            Ok(AppEvent::UpdateMappings(new_mappings)) => {
                state.mappings = new_mappings;
            }

            Ok(AppEvent::SetDeskIp(ip)) => {
                state.desk_ip = ip;
            }

            Ok(AppEvent::Stop) | Err(_) => {
                break;
            }
        }
    }
}

/// Formats a command formatted for Chamsys desks in rx (no header) mode
pub fn translate_midi_to_chamsys_command(message: &[u8], state: &mut AppState) -> Result<Option<String>, ProgramError> {
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
    if is_on_status(status)  {
        command_letter = "A";

    // Note off for all channels
    } else if is_off_status(status) {
        command_letter = "R";

    // MOD WHEEL (LOL)
    } else if status == 176 {
        return Ok(Some(format!("{},{}L", state.previous_playback, velocity)))
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
    state.previous_playback = playback_number;

    // Send back the formatted command
    Ok(Some(format!("{}{}", playback_number, command_letter)))
}

fn send_magicq_command(
    socket: &UdpSocket,
    command: &str,
    state: &AppState,
) -> Result<String, ProgramError> {
    let target = SocketAddrV4::new(state.desk_ip, CHAMSYS_PORT);

    match socket.send_to(&command.as_bytes().to_vec(), target) {
        Ok(_) => (),
        Err(e) => return_err!(format!("Failed to send: {}", e))
    }

    Ok(format!("Command '{}' sent to {}", command, target))
}


