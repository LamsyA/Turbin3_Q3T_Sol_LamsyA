use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
    //  solana_program::{ program::invoke, system_instruction}
};
use anchor_spl::{
    metadata::Metadata,
    token::{mint_to, MintTo, Token},
};
use mpl_token_metadata::{
    instructions::{
        CreateMetadataAccountV3Cpi, CreateMetadataAccountV3CpiAccounts,
        CreateMetadataAccountV3InstructionArgs,
    },
    types::{Collection, Creator, DataV2},
};
// use crate::error::CustomError;
use crate::{
    error::CustomError,
    state::{Event, Ticket},
};

#[derive(Accounts)]
#[instruction(event_name: String)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    /// CHECK: This is safe because we trust the organizer account provided by the client
    #[account(mut)]
    pub organizer: UncheckedAccount<'info>,
    /// CHECK: This is safe because we trust the vault account provided by the client
    #[account(mut)]
    pub mint_authority: UncheckedAccount<'info>,
    /// CHECK: This is safe because we trust the organizer account provided by the client
    #[account(mut)]
    pub taker_ata: UncheckedAccount<'info>,
    ///CHECK: This is safe because we trust the organizer account provided by the client
    #[account(mut)]
    pub vault_ata: UncheckedAccount<'info>,
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub ticket: Account<'info, Ticket>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
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

    pub fn mint_new_ticket(&mut self, event_name: String) -> Result<()> {
        // get event
        if self.event.event_name != event_name {
            return Err(CustomError::EventDoesNotHaveTickets.into());
        }

        if self.ticket.max_supply == 0 {
            return Err(CustomError::EventHasMaxSupply.into());
        }
        self.ticket.nft_mint = Some(self.vault_ata.key());
        self.ticket.max_supply -= 1;

        let now = Clock::get()?.unix_timestamp as i64;

        self.ticket.taker = self.taker.key();
        self.ticket.purchased_date = now;
        // self.ticket.bump = bumps.ticket;

        msg!("Creating Mint Account...");
        let account = MintTo {
            mint: self.vault_ata.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };
        msg!("Initializing account...");
        // seed
        // let seeds = &[
        //     b"vault",
        //     event_name.as_str().as_bytes(),
        //     &[self.ticket.bump],
        // ];
        // let signer = &[&seeds[..]];

        let seeds = &[b"vault", event_name.as_bytes(), &[]];
        let signer = &[&seeds[..]];
        let cpi_context =
            CpiContext::new_with_signer(self.token_program.to_account_info(), account, signer);
        msg!("Minting NFT...");

        // let supply = self.event.max_supply.into();
        mint_to(cpi_context, 1)?;
        msg!("creators ..........");
        let creator = vec![Creator {
            address: self.taker.key(),
            verified: true,
            share: 100,
        }];
        msg!("Creating Metadata Account...");
        CreateMetadataAccountV3Cpi::new(
            &self.token_metadata_program.to_account_info(),
            CreateMetadataAccountV3CpiAccounts {
                metadata: &self.token_metadata_program.to_account_info(),
                mint: &self.vault_ata.to_account_info(),
                mint_authority: &self.mint_authority.to_account_info(),
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
                    uses: None,
                },
                is_mutable: true,
                collection_details: None,
            },
        )
        .invoke_signed(signer)?;

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
