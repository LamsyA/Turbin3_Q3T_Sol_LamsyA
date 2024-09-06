use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
    //  solana_program::{ program::invoke, system_instruction}
};
use anchor_spl::{
    token::{transfer_checked, Token, TokenAccount, TransferChecked},
    token_interface::{Mint, TokenInterface},
};

// use crate::error::CustomError;
use crate::{
    error::CustomError,
    state::{event, Ticket},
    Event,
};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    /// CHECK: This is safe because we trust the organizer account provided by the client
    #[account(mut)]
    pub organizer: AccountInfo<'info>,
    /// CHECK: This is safe because we trust the vault account provided by the client
    #[account(mut)]
    pub vault: AccountInfo<'info>,
    /// CHECK: This is safe because we trust the organizer account provided by the client
    #[account(mut)]
    pub taker_ata: AccountInfo<'info>,
    /// CHECK: This is safe because we trust the organizer account provided by the client
    #[account(mut)]
    pub vault_ata: AccountInfo<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub ticket: Account<'info, Ticket>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

impl<'info> Purchase<'info> {
    pub fn send_sol(&mut self, event_name: String) -> Result<()> {
        if self.event.event_name != event_name {
            return Err(CustomError::EventDoesNotHaveTickets.into());
        }
        if self.ticket.max_supply == 0 {
            return Err(CustomError::EventHasMaxSupply.into());
        }
        let accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.organizer.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), accounts);
        let amount = self.event.ticket_price as u64;
        transfer(cpi_context, amount)?;
        Ok(())
    }
    //     pub fn purchase_ticket(&mut self, event_name: String, bumps: &PurchaseBumps) -> Result<()> {
    //         // get event

    //         if self.event.event_name != event_name  {
    //             return Err(CustomError::EventDoesNotHaveTickets.into());

    //         }997895185

    //         if self.ticket.max_supply == 0 {
    //             return Err(CustomError::EventHasMaxSupply.into());
    //         }
    //         self.ticket.nft_mint = None;
    //         self.ticket.max_supply -= 1;

    //         let now = Clock::get()?.unix_timestamp as i64;

    //         self.ticket.taker =  self.taker.key();
    //         self.ticket.purchased_date = now;
    //         self.ticket.bump = bumps.ticket;

    //         // let signer_seeds = &[
    //         //     b"ticket",
    //         //     self.vault.to_account_info().key.as_ref(),
    //         //     self.organizer.to_account_info().key.as_ref(),
    //         //     &[self.ticket.bump],
    //         // ];

    //         let signer_seeds = &[
    //             b"vault",
    //             self.event.event_name.as_bytes().as_ref(),
    //             // &[self.event.bump],
    //         ];

    //         let signer = &[&signer_seeds[..]];
    // msg!("test...........................");
    //         let cpi_context = TransferChecked{
    //             from: self.vault.to_account_info(),
    //             mint: self.vault_ata.to_account_info(),
    //             to: self.taker_ata.to_account_info(),
    //             authority: self.vault.to_account_info()
    //         };

    //         msg!("signing ................");
    //         let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_context, signer);
    //        msg!("transfer ................");
    //         transfer_checked(cpi_context, self.event.ticket_price as u64, 9)?;
    //         // let lamports = self.event.ticket_price * 10u16.pow(9);
    //         // let instruction = &system_instruction::transfer(&self.taker.key(), &self.event.organizer.key(), lamports as u64);

    //         // invoke(instruction,
    //         //     &[
    //         //         self.taker_ata.to_account_info(),
    //         //         self.vault.to_account_info(),
    //         //         self.system_program.to_account_info(),
    //         //     ],
    //         // )?;

    //         Ok(())

    // }
}
