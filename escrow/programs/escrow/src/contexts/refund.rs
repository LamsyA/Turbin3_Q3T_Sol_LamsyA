use anchor_lang:: prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked,CloseAccount, close_account, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::Escrow;

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
        mint::token_program = token_program)]
    mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program)]
    maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, 
        close = maker,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump)]
    escrow: Account<'info, Escrow>,
    #[account(mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program)]
    vault: InterfaceAccount<'info, TokenAccount>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl <'info> Refund<'info> {
    pub fn withdraw_and_close(&mut self) -> Result<()> {
       
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];
         let accounts = TransferChecked {
             from: self.vault.to_account_info(),
             mint: self.mint_a.to_account_info(),
             to: self.maker_ata_a.to_account_info(),
             authority: self.escrow.to_account_info(),
         };
         let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), accounts, &signer_seeds);
         transfer_checked(ctx, self.escrow.receive, self.mint_a.decimals)?;
         
         let accounts = CloseAccount {
             account: self.vault.to_account_info(),
             destination: self.maker.to_account_info(),
             authority: self.escrow.to_account_info(),
    };

    let ctx = CpiContext::new_with_signer(
        self.token_program.to_account_info(), accounts, &signer_seeds);
    close_account(ctx)?;
    Ok(())
    }

   
}