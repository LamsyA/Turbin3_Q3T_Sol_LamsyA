use anchor_lang::{prelude::*,
    //  solana_program::{ program::invoke, system_instruction}
    system_program::{transfer, Transfer},

    };
use anchor_spl::{metadata::Metadata, token::{mint_to, MintTo, TokenAccount}, token_interface::{Mint, TokenInterface}};
use mpl_token_metadata::{instructions::{CreateMetadataAccountV3Cpi, CreateMetadataAccountV3CpiAccounts, CreateMetadataAccountV3InstructionArgs}, 
types::{
    Collection, 
    Creator, 
    DataV2,
}};


// use crate::error::CustomError;
use crate::{error::CustomError, state::{event, Ticket}};

#[derive(Accounts)]
#[instruction(event_name: String)]
pub struct CreateTicket<'info> {
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
    #[account(
        seeds = [b"vault", event_name.as_str().as_bytes()],
        bump ,
    )]
    pub vault_ata: InterfaceAccount<'info, Mint>,
    #[account(mut, 
        associated_token::mint = vault_ata,
        associated_token::authority = vault,
        associated_token::token_program = token_program
    )]

    pub vault: Account<'info, TokenAccount>,
    pub event: Account<'info, event::Event>,
    pub token_program: Interface<'info, TokenInterface>,
    pub token_metadata_program: Program<'info, Metadata>,

    pub system_program: Program<'info, System>,
}

impl <'info> CreateTicket<'info> {
    pub fn send_sol(&mut self, event_name: String) -> Result<()> {
        if self.event.event_name != event_name  {
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
    pub fn create_ticket(&mut self, event_name: String, bumps: &CreateTicketBumps) -> Result<()> {
       
        // get event
        if self.event.event_name != event_name  {
            return Err(CustomError::EventDoesNotHaveTickets.into());
        }

        if self.ticket.max_supply == 0 {
            return Err(CustomError::EventHasMaxSupply.into());
        }
        self.ticket.nft_mint = Some(self.vault_ata.key());
        self.ticket.max_supply -= 1;
        
        let now = Clock::get()?.unix_timestamp as i64;

        self.ticket.taker =  self.taker.key();
        self.ticket.purchased_date = now;
        self.ticket.bump = bumps.ticket;

        msg!("Creating Mint Account...");
        let account = MintTo {
            mint: self.vault_ata.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.vault.to_account_info(),
        };
        msg!("Minting NFT...");
        // seed
        let seeds = &[b"vault", event_name.as_str().as_bytes(), &[bumps.ticket]];
        let signer = &[&seeds[..]];
        let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), account, signer);

        // let supply = self.event.max_supply.into();
        mint_to(cpi_context, 1)?;
        let creator = vec![
            Creator {
                address: self.taker.key(),
                verified: true,
                share: 100,
            },
        ];

        let metadata_account = CreateMetadataAccountV3Cpi::new(
            &self.token_metadata_program.to_account_info(),
            CreateMetadataAccountV3CpiAccounts {
                metadata: &self.token_metadata_program.to_account_info(),
                mint: &self.vault_ata.to_account_info(),
                mint_authority: &self.vault.to_account_info(),
                payer: &self.taker.to_account_info(),
                update_authority: (&self.organizer.to_account_info(), true),
                system_program: &self.system_program.to_account_info(),
                rent: None,
            }, 
            CreateMetadataAccountV3InstructionArgs {
                data: DataV2 {
                    name: self.event.event_name.clone(),
                    symbol: self.event.symbol.clone(),
                    uri: self.event.uri.clone(),
                    seller_fee_basis_points: 0,
                    creators: Some(creator),
                    collection: Some(Collection {
                        verified: false,
                        key: self.event.key(),
                    }),
                    uses: None
                },
                is_mutable: true,
                collection_details: None,
            }
        );
        
        Ok(())
    
}

}

