use anchor_lang::prelude::*;

declare_id!("71Q9HVkVPD2Kuq8K5eXktSrfemg8RcKLAewEBjCX9YMT");

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
