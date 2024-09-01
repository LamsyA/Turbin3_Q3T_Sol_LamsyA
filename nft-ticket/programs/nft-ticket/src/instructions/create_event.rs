use anchor_lang::prelude::*;

use crate::error::CustomError;
use crate::state::Event;
// use crate::Ticket;

#[derive(Accounts)]
#[instruction(event_name: String)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub organizer: Signer<'info>,

    #[account(
        init,
        payer = organizer,
        space = Event::INIT_SPACE,
        seeds = [b"event", event_name.as_str().as_bytes()],
        bump,
    )]
    pub event: Account<'info, Event>,
    // pub ticket: Account<'info, Ticket>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateEvent<'info> {
    pub fn create_event(
        &mut self,
        event_name: String,
        ticket_price: u16,
        date: i64,
        max_supply: u64,
        description: String,
        bumps: &CreateEventBumps,
    ) -> Result<()> {
        require!(max_supply > 0, CustomError::InvalidMaxSupply);
        self.event.set_inner(Event {
            organizer: self.organizer.key(),
            ticket_price,
            max_supply,
            bump: bumps.event,
            date,
            event_name,
            description,
        });

        // self.ticket.event = self.event.event_name.clone();

        Ok(())
    }
}
