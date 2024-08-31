use anchor_lang::prelude::*;

#[account]
pub struct Event {
    pub organizer: Pubkey,
    pub ticket_price: u16,
    pub max_supply: u64,
    pub bump: u8,
    pub date: i64,
    pub event_name: String,
    pub description: String,
}

impl Space for Event {
    const INIT_SPACE: usize = 8 + 32 + 2 + 8 + 1 + 8 + (32 * 4) + (32 * 4);
}
