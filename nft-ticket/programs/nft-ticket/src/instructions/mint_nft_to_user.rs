use anchor_lang::{
    prelude::*,
    //  solana_program::{ program::invoke, system_instruction}
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{Mint, Token, TokenAccount},
};
use mpl_token_metadata::accounts::{MasterEdition, Metadata as MetadataAccount};

use crate::state::Ticket;
use crate::{error::CustomError, Event};

#[derive(Accounts)]
#[instruction(event_name: String)]

pub struct MintNftToUser<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    pub organizer: Signer<'info>,

    #[account(
        // init,
        // payer = taker,
        mint::decimals = 0,
        mint::authority = vault,
        mint::freeze_authority = vault
    )]
    pub mint: Box<Account<'info, Mint>>,

    // #[account(
    //     init_if_needed,
    //     payer = taker,
    //     associated_token::mint = mint,
    //     associated_token::authority = vault
    // )]
    // pub vault_ata: Box<Account<'info, TokenAccount>>,
    #[account(mut)]
    pub vault: Signer<'info>,

    #[account(init_if_needed,
        payer = taker,
        associated_token::mint = taker,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]
    pub taker_ata: Account<'info, TokenAccount>,

    // #[account(mut)]
    // pub event_creator: Signer<'info>,

    // pub collection: Account<'info, Mint>,
    /// CHECK: Metaplex will check this
    #[account(
        mut,
        address = MetadataAccount::find_pda(&mint.key()).0,
    )]
    pub metadata_account: AccountInfo<'info>,

    /// CHECK: Metaplex will check this
    #[account(
        mut,
        address = MasterEdition::find_pda(&mint.key()).0,
    )]
    pub master_edition_account: AccountInfo<'info>,
    pub event: Box<Account<'info, Event>>,

    pub ticket: Box<Account<'info, Ticket>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> MintNftToUser<'info> {
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
        let amount = self.event.ticket_price.checked_mul(10000).unwrap();
        transfer(cpi_context, amount.into())?;
        Ok(())
    }
}

// EbLh65VJjsT5BAFrQVBkuUuiNbJNtFNDSzVgrvSCMHu9
