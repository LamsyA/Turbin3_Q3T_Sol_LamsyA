use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{mint_to, Mint, MintTo, TokenAccount, TokenInterface},
};

use crate::state::{StakeConfig, UserAccount};

#[derive(Accounts)]

pub struct Claim<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut,
        seeds =[b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = ["config".as_ref()],
        bump = config.bump,
        )]
    pub config: Account<'info, StakeConfig>,
    #[account(
        seeds = [b"reward".as_ref(), config.key().as_ref()],
        bump = config.reward_bump,
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user,

    )]
    pub reward_ata: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_account = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.reward_ata.to_account_info(),
            authority: self.config.to_account_info(),
        };

        let signer_seeds = &[b"config".as_ref(), &[self.config.bump]];

        let signer = [&signer_seeds[..]];
        let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_account, &signer);
        mint_to(
            cpi_context,
            self.user_account.points as u64 * 10_u64.pow(self.reward_mint.decimals as u32),
        )?;
        self.user_account.points = 0;
        Ok(())
    }
}
