use anchor_lang::prelude::*;

declare_id!("BPSkKKbLFseoabHF5cwdih4V2PUbmnc44j488XSLGDm5");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn make(ctx: Context<Initialize>, seed: u64, amount: u64, receive: u64) -> Result<()> {
        ctx.accounts.save_escrow(seed, receive, ctx.bumps)?;
        ctx.accounts.deposit_to_vault(amount)
        Ok(())
    }
    pub fn take(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
    pub fn refund(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
