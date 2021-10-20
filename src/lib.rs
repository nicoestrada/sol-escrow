//lib.rs is used for registering modules
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// the entrypoint function is found in entrypoint.rs
pub mod entrypoint;

//the instruction function
pub mod instruction;

pub mod state;
pub mod processor;
pub mod error;