use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::time::Duration;
use crate::midi_io::{get_midi_input, get_midi_input_port, get_midi_output};
use crate::{return_err, ProgramError};
use crate::midi_translator::translate_midi_to_chamsys_command;

// Default port MagicQ listens on for remote control UDP
const CHAMSYS_PORT: u16 = 6553;
const SOCKET_ADDR: &'static str = "127.0.0.1:3400";

pub fn midi_through_to_chamsys() -> Result<(), ProgramError> {
    println!("\nRUNNING CHAMSYS CONTROL PROGRAM");
    let mut conn_out = get_midi_output()?;
    let midi_in = get_midi_input()?;
    let in_port = get_midi_input_port(&midi_in)?;

    // Try discovering the desk on the local network
    match discover_desk(Duration::from_secs(2))? {
        Some(addr) => {
            println!("Found MagicQ at {:?}", addr);

            let mut seq = 0;

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
                            send_magicq_command(&addr, c, true, seq);
                            seq += 1;
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



        }
        None => println!("No MagicQ desk found"),
    }

    Ok(())
}

// Builds a very simple CREP packet
fn build_crep_packet(seq: u8, data_bytes: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();

    // CREP header (Little endian representation on the wire)
    // 'C' 'R' 'E' 'P' as bytes
    buf.extend_from_slice(b"CREP");

    // version = 0
    buf.extend_from_slice(&0u16.to_le_bytes());

    // fwd sequence
    buf.push(seq);

    // bkwd sequence (we don't expect responses here, so 0)
    buf.push(0);

    // length of the data
    buf.extend_from_slice(&(data_bytes.len() as u16).to_le_bytes());

    // the actual data
    buf.extend_from_slice(data_bytes);

    buf
}

/// Try to find a MagicQ on the local network
///
/// Binds to "0.0.0.0:0", sets broadcast, and sends an empty packet that prompts
/// the desk to respond (if any implementation does).
///
/// Returns the first responding address or error.
fn discover_desk(timeout: Duration) -> Result<Option<SocketAddr>, ProgramError> {
    let socket = match UdpSocket::bind(SOCKET_ADDR) {
        Ok(s) => s,
        Err(e) => return_err!(format!("failed to bind socket: {}", e)),
    };

    match socket.set_broadcast(true) {
        Ok(_) => (),
        Err(e) => return_err!(format!("failed to set broadcast: {}", e)),
    }

    match socket.set_read_timeout(Some(timeout)) {
        Ok(_) => (),
        Err(e) => return_err!(format!("failed to set read timeout: {}", e)),
    }

    // We can send an (empty or simple request) to broadcast
    let broadcast_addr = SocketAddrV4::new(Ipv4Addr::BROADCAST, CHAMSYS_PORT);

    let _ = match socket.send_to(b"", broadcast_addr) {
        Ok(r) => r,
        Err(e) => return_err!(format!("failed to send broadcast: {}", e)),
    };

    let mut buf = [0u8; 2048];
    match socket.recv_from(&mut buf) {
        Ok((_, addr)) => Ok(Some(addr)),
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => Ok(None),
        Err(e) => Err(ProgramError::new(format!("Error with receiving the socket: {e}"))),
    }
}

/// Sends a formatted command (optionally with CREP header)
///
/// Example `command_text`: `"1A"` (activate playback 1)
fn send_magicq_command(
    desk_addr: &SocketAddr,
    command_text: &str,
    use_crep: bool,
    seq: u8,
) -> Result<(), ProgramError> {
    let socket = match UdpSocket::bind(SOCKET_ADDR) {
        Ok(s) => s,
        Err(e) => return_err!(format!("failed to bind socket: {}", e)),
    };

    let bytes = command_text.as_bytes();

    let payload = if use_crep {
        build_crep_packet(seq, bytes)
    } else {
        bytes.to_vec()
    };

    match socket.send_to(&payload, desk_addr) {
        Ok(_) => (),
        Err(e) => return_err!(format!("failed to send command: {}", e)),
    }

    Ok(())
}


