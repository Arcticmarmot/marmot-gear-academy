// build.rs
// Where ProgramMetadata is your metadata structure

use pebbles_game_io::PebblesMetadata;

fn main() {
    gear_wasm_builder::build_with_metadata::<PebblesMetadata>();
}