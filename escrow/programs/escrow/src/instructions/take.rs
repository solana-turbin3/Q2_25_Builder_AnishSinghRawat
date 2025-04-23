use anchor_lang::{accounts, prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token_interface::{TokenAccount, TokenInterface, Mint, TransferChecked, transfer_checked}};
use crate::{escrow, state::Escrow};


#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        address = escrow.maker
    )]
    pub maker: SystemAccount<'info>,

    #[account(
        mint::token_program=token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program=token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program 
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}   

#[derive(Accounts)]
#[instruction(seed: u64)]

pub struct Close<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        mut,
        close = taker,
        seeds = [b"escrow", taker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mint::token_program=token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program 
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}


impl<'info> Take<'info> {
    pub fn deposit(&mut self, deposit: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let transfer_Accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.maker.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, transfer_Accounts);

        transfer_checked(cpi_ctx, deposit, self.mint_b.decimals)
    }

    pub fn withdraw(&mut self, withdraw: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let transfer_Accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.maker.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, transfer_Accounts);

        transfer_checked(cpi_ctx, withdraw, self.mint_a.decimals)
    }

}

impl<'info> Close<'info> {
    pub fn close(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let transfer_Accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker.to_account_info(),
            authority: self.escrow.to_account_info()
        };
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            self.escrow.maker.as_ref(),  
            &self.escrow.seed.to_le_bytes(),
            &[self.escrow.bump]
        ]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, transfer_Accounts, signer_seeds);
        transfer_checked(cpi_ctx, self.vault.amount, self.mint_a.decimals)
 
    }
}

