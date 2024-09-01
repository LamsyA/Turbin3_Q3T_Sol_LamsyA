use anchor_lang::{prelude::*,
    //  solana_program::{ program::invoke, system_instruction}
    };
use anchor_spl::{token::{transfer_checked, TokenAccount, TransferChecked}, token_interface::{Mint, TokenInterface}};

// use crate::error::CustomError;
use crate::{error::CustomError, state::{event, Ticket}};

#[derive(Accounts)]
#[instruction(event_name: String)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub organizer: Signer<'info>,
    

    #[account(
        init,
        payer = taker,
        space = Ticket::INIT_SPACE,
        seeds = [b"ticket", event_name.as_str().as_bytes()],
        bump,
    )]
    pub ticket: Account<'info, Ticket>,
    #[account(mut, 
        associated_token::mint = taker,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub taker_ata: Account<'info, TokenAccount>,
    pub vault_mint: InterfaceAccount<'info, Mint>,
    #[account(mut, 
        associated_token::mint = vault_mint,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub vault: Account<'info, TokenAccount>,
    pub event: Account<'info, event::Event>,
    pub token_program: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,
}

impl <'info> Purchase<'info> {
    pub fn purchase_ticket(&mut self, event_name: String, bumps: &PurchaseBumps) -> Result<()> {
        // get event

        if self.event.event_name != event_name  {
            return Err(CustomError::EventDoesNotHaveTickets.into());
            
        }

        if self.ticket.max_supply == 0 {
            return Err(CustomError::EventHasMaxSupply.into());
        }
        self.ticket.nft_mint = None;
        self.ticket.max_supply -= 1;
        
        let now = Clock::get()?.unix_timestamp as i64;

        self.ticket.taker =  self.taker.key();
        self.ticket.purchased_date = now;
        self.ticket.bump = bumps.ticket;


        let signer_seeds = &[
            b"ticket",
            self.vault.to_account_info().key.as_ref(),
            self.organizer.to_account_info().key.as_ref(),
            &[self.ticket.bump],
        ];
        let signer = &[&signer_seeds[..]];

        let cpi_context = TransferChecked{
            from: self.vault.to_account_info(),
            mint: self.vault_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.vault.to_account_info()
        };
        let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_context, signer);
        transfer_checked(cpi_context, self.event.ticket_price as u64, 9)?;
        // let lamports = self.event.ticket_price * 10u16.pow(9);
        // let instruction = &system_instruction::transfer(&self.taker.key(), &self.event.organizer.key(), lamports as u64);

        // invoke(instruction,
        //     &[
        //         self.taker_ata.to_account_info(),
        //         self.vault.to_account_info(),
        //         self.system_program.to_account_info(),
        //     ], 
        // )?;

        Ok(())
    
    
}
}

