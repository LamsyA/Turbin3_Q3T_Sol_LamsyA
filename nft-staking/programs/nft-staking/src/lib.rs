use anchor_lang::prelude::*;
pub mod constant;
pub mod error;
pub mod instructions;
pub mod state;

pub use constant::*;
pub use instructions::*;
pub use state::*;

declare_id!("Ggrqptteo29LjppFVxLEsJ34TcohX7U6oDk5G3bNzK37");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_time: u32,
    ) -> Result<()> {
        ctx.accounts
            .init_config(points_per_stake, max_stake, freeze_time, &ctx.bumps)
    }
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.init_user(&ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}
