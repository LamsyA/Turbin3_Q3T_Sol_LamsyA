use anchor_lang::prelude::*;

#[account]
// #[derive(InitSpace)]
pub struct StakeConfig {
    pub point_per_stake: u8,
    pub max_stake: u8,
    pub freeze_time: u32,
    pub reward_bump: u8,
    pub bump: u8,
}

impl Space for StakeConfig {
    const INIT_SPACE: usize = 8 + 1 + 1 + 4 + 1 + 1;
}
