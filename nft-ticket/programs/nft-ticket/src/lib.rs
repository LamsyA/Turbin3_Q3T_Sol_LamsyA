use anchor_lang::prelude::*;
declare_id!("GVpt8Jt7ZeRNccwBThid9eiPRquDP1KiQHs3n3ADdbYn");

pub mod error;

pub mod instructions;
pub use instructions::*;

pub mod state;

pub use state::*; // Explicit import to resolve ambiguity

#[program]
pub mod nft_ticket {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        event_name: String,
        ticket_price: u64,
        date: i64,
        max_supply: u16,
        description: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.create_event(
            event_name,
            ticket_price,
            date,
            max_supply,
            description,
            symbol,
            uri,
            &ctx.bumps,
        )
    }
    pub fn create_collection(ctx: Context<CreateCollection>) -> Result<()> {
        ctx.accounts.mint_nft()?;
        ctx.accounts.create_collection()
    }

    pub fn create_nft_for_collection(ctx: Context<CreateNft>, event_name: String) -> Result<()> {
        ctx.accounts.send_sol(event_name.clone())?;
        ctx.accounts.mint_ticket_as_nft(event_name)
    }
}
