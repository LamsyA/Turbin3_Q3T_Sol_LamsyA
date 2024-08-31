use anchor_lang::prelude::*;

#[account]
pub struct Ticket {
    pub event: String,
    pub price: u16,
    pub purchased_date: i64,
    pub taker: Pubkey,
    pub nft_mint: Option<Pubkey>,
    pub bump: u8,
    pub max_supply: u64,
}
impl Space for Ticket {
    const INIT_SPACE: usize = 8 + 32 + 2 + 8 + 32 + 32 + 1 + 8;
}
