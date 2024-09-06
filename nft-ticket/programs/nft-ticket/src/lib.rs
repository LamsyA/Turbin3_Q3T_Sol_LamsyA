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
    pub fn create_nft(
        ctx: Context<CreateNft>,
        // name: String,
        // symbol: String,
        // uri: String,
    ) -> Result<()> {
        ctx.accounts.mint_nft()?;
        ctx.accounts.create_nft()
    }

    pub fn purchase_ticket(ctx: Context<Purchase>, event_name: String) -> Result<()> {
        // ctx.accounts.purchase_ticket(event_name, &ctx.bumps)
        ctx.accounts.send_sol(event_name)
    }
    // pub fn create_ticket(ctx: Context<CreateTicket>, event_name: String) -> Result<()> {
    //     ctx.accounts.send_sol(event_name.clone())?;
    //     ctx.accounts.create_ticket(event_name, &ctx.bumps)
    // }
    pub fn mint_nft_to_user(ctx: Context<MintNftToUser>, event_name: String) -> Result<()> {
        ctx.accounts.send_sol(event_name)
    }
}
