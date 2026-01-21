use std::io::stdin;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use crate::midi_io::{get_midi_input, get_midi_input_port, get_midi_output};
use crate::{return_err, ProgramError};
use crate::midi_translator::translate_midi_to_chamsys_command;

// Default port MagicQ listens on for remote control UDP
const CHAMSYS_PORT: u16 = 6553;
pub fn midi_through_to_chamsys() -> Result<(), ProgramError> {
    println!("\nRUNNING CHAMSYS MIDI CONTROL");
    let mut conn_out = get_midi_output()?;
    let midi_in = get_midi_input()?;
    let in_port = get_midi_input_port(&midi_in)?;

    let socket_address = SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::BROADCAST,
        CHAMSYS_PORT,
    ));

    let socket = match open_chamsys_socket(&socket_address) {
        Ok(s) => s,
        Err(e) => return_err!(&format!("failed to open socket: {}", e))
    };

    let mut seq_forwards = 0;
    let mut seq_backwards = 0;

    let _conn_in = match midi_in.connect(
        &in_port,
        "midir-read-input",
        move |stamp, message, _| {

            // RUN ANY INPUT TESTS IN HERE
            let command = translate_midi_to_chamsys_command(message);

            println!("MIDI {}: {:?}", stamp, message);

            match command {
                Ok(c) => {
                    println!("Sent command: {} to Chamsys", c);
                    match send_magicq_command(&socket, &socket_address, c, true, &mut seq_forwards, &mut seq_backwards) {
                        Ok(_) => (),
                        Err(e) => println!("{}", e)
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

fn open_chamsys_socket(socket_address: &SocketAddr) -> Result<UdpSocket, ProgramError> {
    let socket = UdpSocket::bind(socket_address)
        .map_err(|e| ProgramError::new(format!(
            "failed to bind socket: {e}"
        )))?;

    socket.set_broadcast(true)
        .map_err(|e| ProgramError::new(format!(
            "failed to enable broadcast: {e}"
        )))?;

    // Optional: non-blocking if you later want recv()
    // socket.set_nonblocking(true)?;

    Ok(socket)
}

/// Sends a formatted command (optionally with CREP header)
///
/// Example `command_text`: `"1A"` (activate playback 1)
fn send_magicq_command(
    socket: &UdpSocket,
    target: &SocketAddr,
    command_text: &str,
    use_crep: bool,
    seq_forwards: &mut u8,
    seq_backwards: &mut u8,
) -> Result<(), ProgramError> {

    let payload = if use_crep {
        build_crep_packet(*seq_forwards, *seq_backwards, command_text.as_bytes())
    } else {
        command_text.as_bytes().to_vec()
    };

    socket.send_to(&payload, target)
        .map_err(|e| ProgramError::new(format!(
            "failed to send command: {e}"
        )))?;

    *seq_forwards = seq_forwards.wrapping_add(1);

    Ok(())
}


