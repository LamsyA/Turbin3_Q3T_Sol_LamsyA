use anchor_lang::prelude::*;
pub mod error;
pub use error::*;
pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;
declare_id!("APgzc5wu25qDa9oXs21mn2JGmsTnkf2tiZTvYGEoxtzo");

#[program]
pub mod nft_ticketing {
    use super::*;

    pub fn create_event(
        ctx: Context<CreateEvent>,
        event_name: String,
        ticket_price: u16,
        date: i64,
        max_supply: u16,
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

    // pub fn buy_ticket(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }
    // pub fn create_event(ctx: Context<Initialize>) -> Result<()> {
    //     msg!("Greetings from: {:?}", ctx.program_id);
    //     Ok(())
    // }
}
