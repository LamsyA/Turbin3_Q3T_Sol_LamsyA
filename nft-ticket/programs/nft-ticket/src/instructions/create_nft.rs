use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::metadata::{
    create_metadata_accounts_v3,
    mpl_token_metadata::{
        instructions::{
            CreateMasterEditionV3Cpi, CreateMasterEditionV3CpiAccounts,
            CreateMasterEditionV3InstructionArgs,
        },
        types::{Creator, DataV2},
    },
    CreateMetadataAccountsV3, Metadata,
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use mpl_token_metadata::{
    accounts::{MasterEdition, Metadata as MetadataAccount},
    types::Collection,
};

use crate::{error::CustomError, Event, Ticket};

#[derive(Accounts)]
pub struct CreateNft<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        init,
        payer = taker,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub collection_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = taker,
        associated_token::mint = collection_mint,
        associated_token::authority = mint_authority
    )]
    pub destination: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: metaplex will check this
    #[account(
        mut,
        address = MetadataAccount::find_pda(&collection_mint.key()).0,
    )]
    pub metadata: UncheckedAccount<'info>,

    /// CHECK: Metaplex will check this
    #[account(
        mut,
        address = MasterEdition::find_pda(&collection_mint.key()).0,
    )]
    pub master_edition: UncheckedAccount<'info>,

    pub event: Box<Account<'info, Event>>,
    #[account(mut)]
    pub ticket: Box<Account<'info, Ticket>>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_metadata_program: Program<'info, Metadata>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateNft<'info> {
    pub fn send_sol(&mut self, event_name: String) -> Result<()> {
        if self.event.event_name != event_name {
            return Err(CustomError::EventDoesNotHaveTickets.into());
        }
        if self.ticket.max_supply == 0 {
            return Err(CustomError::EventHasMaxSupply.into());
        }
        let accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.mint_authority.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), accounts);
        let amount = self.event.ticket_price as u64;
        transfer(cpi_context, amount)?;
        Ok(())
    }
    pub fn mint_ticket_as_nft(&mut self, event_name: String) -> Result<()> {
        // get event
        if self.event.event_name != event_name {
            return Err(CustomError::EventDoesNotHaveTickets.into());
        }

        if self.ticket.max_supply == 0 {
            return Err(CustomError::EventHasMaxSupply.into());
        }
        self.ticket.nft_mint = Some(self.destination.key());
        self.ticket.max_supply -= 1;

        let now = Clock::get()?.unix_timestamp as i64;

        self.ticket.taker = self.taker.key();
        self.ticket.purchased_date = now;

        msg!("Creating Mint Account...");
        let account = MintTo {
            mint: self.collection_mint.to_account_info(),
            to: self.destination.to_account_info(),
            authority: self.mint_authority.to_account_info(),
        };
        msg!("Minting NFT...");
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), account);

        mint_to(cpi_context, 1)?;
        msg!("Creating Metadata Account...");
        let supply = self.event.max_supply as u64;
        let cpi_context = CpiContext::new(
            self.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: self.metadata.to_account_info(),
                mint: self.collection_mint.to_account_info(),
                mint_authority: self.mint_authority.to_account_info(),
                payer: self.taker.to_account_info(),
                update_authority: self.mint_authority.to_account_info(),
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
                address: self.mint_authority.key(),
                verified: true,
                share: 100,
            }]),
            collection: Some(Collection {
                verified: false,
                key: self.collection_mint.key(),
            }),
            uses: None,
        };
        msg!(" V3 Creating Master Edition Account...");
        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

        CreateMasterEditionV3Cpi::new(
            &self.token_metadata_program.to_account_info(),
            CreateMasterEditionV3CpiAccounts {
                edition: &self.master_edition.to_account_info(),
                metadata: &self.metadata.to_account_info(),
                mint: &self.collection_mint.to_account_info(),
                mint_authority: &self.mint_authority.to_account_info(),
                payer: &self.taker.to_account_info(),
                update_authority: &self.mint_authority.to_account_info(),
                system_program: &self.system_program.to_account_info(),
                token_program: &self.token_program.to_account_info(),
                rent: Some(&self.rent.to_account_info()),
            },
            CreateMasterEditionV3InstructionArgs {
                max_supply: Some(supply),
            },
        )
        .invoke()?;
        msg!("V3 Creating Master Edition Account... loading supply...");
        msg!("minted succefully with supply: {}", supply);

        self.ticket.nft_mint = Some(self.mint_authority.key());

        Ok(())
    }
}
