use anchor_lang::prelude::*;

declare_id!("71Q9HVkVPD2Kuq8K5eXktSrfemg8RcKLAewEBjCX9YMT");

pub mod error;
pub mod instructions;
pub mod state;

pub use error::*;
pub use instructions::*;
pub use state::*;

#[program]
pub mod nft_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u16, name: String) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }
}
