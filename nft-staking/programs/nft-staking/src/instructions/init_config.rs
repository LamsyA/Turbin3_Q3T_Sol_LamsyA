use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::state::StakeConfig;

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = ["config".as_ref()],
        bump,
        space = StakeConfig::INIT_SPACE,)]
    pub config: Account<'info, StakeConfig>,
    
    #[account(
        init, 
        payer = admin,
        seeds = [b"reward".as_ref(), config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    
    pub reward_mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program : Program<'info, Token>,
}


impl<'info> InitializeConfig<'info> {
    pub fn init_config(&mut self, point_per_stake: u8,
    max_stake: u8, freeze_time: u32, bumps: &InitializeConfigBumps) -> Result<()> {
        self.config.set_inner(StakeConfig{
            point_per_stake,
            max_stake,
            freeze_time,
            reward_bump: bumps.reward_mint,
            bump: bumps.config 
        });
        Ok(())
    }
}


