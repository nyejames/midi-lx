use std::io::stdin;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use crate::midi_io::{get_midi_input, get_midi_input_port, get_midi_output};
use crate::errors::ProgramError;
use crate::midi_translator::translate_midi_to_chamsys_command;
use crate::return_err;

// Default port MagicQ listens on for remote control UDP
const CHAMSYS_PORT: u16 = 6553;
const LOCAL_IP: Ipv4Addr = Ipv4Addr::new(2, 0, 0, 1);
const CHAMSYS_IP: Ipv4Addr = Ipv4Addr::new(2, 0, 0, 35);
const USE_CREP: bool = false;

pub fn midi_through_to_chamsys() -> Result<(), ProgramError> {
    println!("\nRUNNING CHAMSYS MIDI CONTROL");
    let mut conn_out = get_midi_output()?;
    let midi_in = get_midi_input()?;
    let in_port = get_midi_input_port(&midi_in)?;

    println!("Local IP for sending: {}", LOCAL_IP);

    // Try pinging the desk
    if let Ok(_) = std::process::Command::new("ping")
        .arg("2.0.0.35")
        .output() {
        println!("Ping to 2.0.0.35 succeeded");
    } else {
        println!("Ping failed — network config may be wrong");
    }

    let socket = match UdpSocket::bind((LOCAL_IP, 0)) {
        Ok(s) => s,
        Err(e) => return_err!(format!("failed to bind socket: {}", e))
    };

    let mut seq_forwards = 0;
    let mut seq_backwards = 0;

    let mut previous_playback: u8 = 0;

    let _conn_in = match midi_in.connect(
        &in_port,
        "midir-read-input",
        move |stamp, message, _| {

            // RUN ANY INPUT TESTS IN HERE
            let command = translate_midi_to_chamsys_command(message, &mut previous_playback);

            match command {
                Ok(c) => {
                    // Don't send a command if None was returned (wasteful)
                    if let Some(command) = c {
                        match send_magicq_command(&socket, &command, USE_CREP, &mut seq_forwards, &mut seq_backwards) {
                            Ok(details) => println!("{}", details),
                            Err(e) => println!("{}", e)
                        }
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

    // Just wait for the user to press any key to exit
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(e) => return_err!(format!("failed to read line from stdin: {e}")),
    }

    Ok(())
}

// Builds a very simple CREP packet
fn build_crep_packet(
    seq_fwd: u8,
    seq_bkwd: u8,
    payload: &[u8],
) -> Vec<u8> {
    let mut packet = Vec::with_capacity(10 + payload.len());

    // Magic
    packet.extend_from_slice(b"CREP");

    // Version (u16, BE) — always zero
    packet.extend_from_slice(&0u16.to_be_bytes());

    // Sequence numbers
    packet.push(seq_fwd);
    packet.push(seq_bkwd);

    // Payload length (u16, BE)
    packet.extend_from_slice(&(payload.len() as u16).to_be_bytes());

    // Payload
    packet.extend_from_slice(payload);

    packet
}

/// Sends a formatted command (optionally with CREP header)
///
/// Example `command_text`: `"1A"` (activate playback 1)
fn send_magicq_command(
    socket: &UdpSocket,
    command: &str,
    use_crep: bool,
    seq_forwards: &mut u8,
    seq_backwards: &mut u8,
) -> Result<String, ProgramError> {

    let payload = if use_crep {
        build_crep_packet(*seq_forwards, *seq_backwards, command.as_bytes())
    } else {
        // Just send raw command with terminator
        command.as_bytes().to_vec()
    };

    let target = SocketAddrV4::new(CHAMSYS_IP, CHAMSYS_PORT);

    match socket.send_to(&payload, target) {
        Ok(_) => (),
        Err(e) => return_err!(format!("Failed to send: {}", e))
    }

    if use_crep {
        *seq_forwards = seq_forwards.wrapping_add(1);
        *seq_backwards = seq_backwards.wrapping_add(1);
    }

    Ok(format!("Command '{}' sent to {}. Sequence: ({})", command, target, seq_forwards))
}


