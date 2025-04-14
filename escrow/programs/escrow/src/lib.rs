#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instruction::*;
pub use state::*;


declare_id!("8ZXa3zWkvVaNtBEHuTVMsnb6fyKBajuoPRhigyHATHPD");

#[program]
pub mod escrow {
    use super::*;
}

#[derive(Accounts)]
pub struct Initialize {}
