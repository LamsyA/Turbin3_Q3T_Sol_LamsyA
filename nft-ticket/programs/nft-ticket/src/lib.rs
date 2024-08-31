use anchor_lang::prelude::*;
declare_id!("GVpt8Jt7ZeRNccwBThid9eiPRquDP1KiQHs3n3ADdbYn");

pub mod error;

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

#[program]
pub mod nft_ticket {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        event_name: String,
        ticket_price: u16,
        date: i64,
        max_supply: u64,
        description: String,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.create_event(
            event_name,
            ticket_price,
            date,
            max_supply,
            description,
            &ctx.bumps,
        )
    }
    pub fn create_nft(
        ctx: Context<CreateNft>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        ctx.accounts.create_nft(name, symbol, uri)
    }
}
