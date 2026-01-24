use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::sync::mpsc;
use color_print::ceprintln;
use crate::chamsys::{start_midi_to_chamsys_runtime, AppEvent, AppState};
use crate::errors::ProgramError;
use crate::organ::organ_midi::play_organ;

pub mod errors;
mod midi_io;
mod midi_utils;
mod test;
mod chamsys;

mod organ {
    pub mod stops_table;
    pub mod organ_midi;
}
pub mod cli;

#[derive(Clone)]
pub enum LxCommand {
    Activate,
    Deactivate,
    Intensity,
}

pub fn organ_control() {
    match play_organ(false) {
        Ok(_) => (),
        Err(e) => {
            ceprintln!("<red>{}</>", e)
        },
    }
}

pub struct MidiRuntime {
    tx: mpsc::Sender<AppEvent>,
}

impl MidiRuntime {

    pub fn create(
        desk_ip: Ipv4Addr,
        app_ip: Ipv4Addr,
        midi_input: midir::MidiInput,
        selected_midi_port: midir::MidiInputPort,
        midi_through: Option<midir::MidiOutputConnection>,
    ) -> Result<MidiRuntime, ProgramError> {

        start_midi_to_chamsys_runtime(
            AppState::new(desk_ip, app_ip),
            midi_input,
            selected_midi_port,
            midi_through,
        )
    }

    pub fn update_mappings(&self, mappings: HashMap<usize, LxCommand>) {
        let _ = self.tx.send(AppEvent::UpdateMappings(mappings));
    }

    pub fn set_desk_ip(&self, ip: Ipv4Addr) {
        let _ = self.tx.send(AppEvent::SetDeskIp(ip));
    }

    pub fn stop(&self) {
        let _ = self.tx.send(AppEvent::Stop);
    }
}


