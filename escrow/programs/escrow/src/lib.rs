use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("8ZXa3zWkvVaNtBEHuTVMsnb6fyKBajuoPRhigyHATHPD");

#[program]
pub mod escrow {
    use super::*;

    // pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
    //         ctx.accounts.deposit()

    // }

    // pub fn take(ctx: Context<Take>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
    //     ctx.accounts.deposit()

    // }

    // pub fn refund(ctx: Context<Refund>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
    //     ctx.accounts.deposit()

    // }
}

#[derive(Accounts)]
pub struct Initialize {}
