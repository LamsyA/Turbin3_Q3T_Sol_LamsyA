use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    accounts::{MasterEdition, Metadata as MetadataAccount},
    types::{CollectionDetails, Creator, DataV2},
};

use crate::state::Ticket;
use crate::Event;

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = vault,
        mint::freeze_authority = vault
    )]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = vault
    )]
    pub vault_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub vault: Signer<'info>,

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
    #[account(mut)]
    pub ticket: Box<Account<'info, Ticket>>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateCollection<'info> {
    pub fn mint_nft(&mut self) -> Result<()> {
        msg!("Creating Mint Account...");
        let account = MintTo {
            mint: self.mint.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.vault.to_account_info(),
        };
        msg!("Minting NFT...");
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), account);

        // let supply = self.event.max_supply.into();
        mint_to(cpi_context, 1)?;

        Ok(())
    }
    pub fn create_collection(
        &mut self,
        // name: String,
        // symbol: String,
        // uri: String,
        // price: u16, // Specify the ticket price here
    ) -> Result<()> {
        msg!("Creating Metadata Account...");
        let supply = self.event.max_supply.into();
        let cpi_context = CpiContext::new(
            self.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: self.metadata_account.to_account_info(),
                mint: self.mint.to_account_info(),
                mint_authority: self.vault.to_account_info(),
                payer: self.signer.to_account_info(),
                update_authority: self.signer.to_account_info(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
        );
        msg!("Creating Master Edition Account...");
        let data_v2 = DataV2 {
            name: self.event.event_name.clone(),
            symbol: self.event.symbol.clone(),
            uri: self.event.uri.clone(),
            seller_fee_basis_points: 0,
            creators: Some(vec![Creator {
                address: self.signer.key(),
                verified: false,
                share: 100,
            }]),
            collection: None,
            uses: None,
        };
        msg!(" V3 Creating Master Edition Account...");
        create_metadata_accounts_v3(
            cpi_context,
            data_v2,
            false,
            true,
            Some(CollectionDetails::V1 {
                size: self.ticket.max_supply.into(),
            }),
        )?;

        let cpi_context = CpiContext::new(
            self.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: self.master_edition_account.to_account_info(),
                metadata: self.metadata_account.to_account_info(),
                mint: self.mint.to_account_info(),
                mint_authority: self.vault.to_account_info(),
                payer: self.signer.to_account_info(),
                update_authority: self.signer.to_account_info(),
                system_program: self.system_program.to_account_info(),
                token_program: self.token_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
        );
        msg!("V3 Creating Master Edition Account... loading supply...");
        create_master_edition_v3(cpi_context, Some(supply))?;
        self.ticket.price = self.event.ticket_price; // Store the event price in the ticket account
        self.ticket.max_supply = self.event.max_supply;
        // self.ticket.event = self.event.event_name.clone();

        Ok(())
    }
}
