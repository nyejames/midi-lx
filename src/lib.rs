pub mod errors;
mod midi_io;
mod midi_translator;
mod test;
mod chamsys;

mod organ {
    pub mod stops_table;
    pub mod organ_midi;
}
pub mod cli;