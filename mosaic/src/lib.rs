pub mod errors;
pub mod instructions;
pub mod processor;
pub mod seeds;
pub mod state;

use {
    pinocchio::program_entrypoint,
    processor::process_instruction,
    solana_program::{custom_heap_default, custom_panic_default},
};

custom_heap_default!();
custom_panic_default!();
program_entrypoint!(process_instruction);

pinocchio_pubkey::declare_id!("s75D2Kb5WnVBsFQiSLj5E4oRgwDJU63487cSnp2khXh");
