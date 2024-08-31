use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::{get_associated_token_address, AssociatedToken},
    metadata::{
        create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
        CreateMetadataAccountsV3, Metadata,
    },
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};
use mpl_token_metadata::{
    accounts::{MasterEdition, Metadata as MetadataAccount},
    types::DataV2,
};

use crate::state::Ticket;
use crate::{error::CustomError, Event};

#[derive(Accounts)]
pub struct CreateNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = vault,
        mint::freeze_authority = vault
    )]
    pub mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = vault
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub mint_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault: Signer<'info>,

    #[account(mut)]
    pub event_creator: Signer<'info>,

    pub collection: Account<'info, Mint>,

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
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub ticket: Account<'info, Ticket>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateNft<'info> {
    pub fn create_nft(
        &mut self,
        name: String,
        symbol: String,
        uri: String,
        // price: u16, // Specify the ticket price here
    ) -> Result<()> {
        msg!("Creating Mint Account...");
        let account = MintTo {
            mint: self.mint.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.vault.to_account_info(),
        };
        let cpi_context = CpiContext::new(self.token_program.to_account_info(), account);

        let supply = self.event.max_supply;
        mint_to(cpi_context, supply)?;

        msg!("Creating Metadata Account...");

        let cpi_context = CpiContext::new(
            self.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: self.metadata_account.to_account_info(),
                mint: self.mint.to_account_info(),
                mint_authority: self.vault.to_account_info(),
                payer: self.signer.to_account_info(),
                update_authority: self.event_creator.to_account_info(),
                system_program: self.system_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
        );

        let data_v2 = DataV2 {
            name,
            symbol,
            uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

        let cpi_context = CpiContext::new(
            self.token_metadata_program.to_account_info(),
            CreateMasterEditionV3 {
                edition: self.master_edition_account.to_account_info(),
                metadata: self.metadata_account.to_account_info(),
                mint: self.mint.to_account_info(),
                mint_authority: self.vault.to_account_info(),
                payer: self.signer.to_account_info(),
                update_authority: self.event_creator.to_account_info(),
                system_program: self.system_program.to_account_info(),
                token_program: self.token_program.to_account_info(),
                rent: self.rent.to_account_info(),
            },
        );

        create_master_edition_v3(cpi_context, Some(supply))?;
        self.ticket.nft_mint = Some(self.mint.key());
        self.ticket.price = self.event.ticket_price; // Store the event price in the ticket account
        self.ticket.max_supply = self.event.max_supply;
        Ok(())
    }

    pub fn mint_nft_to_user(&mut self, user: AccountInfo<'info>) -> Result<()> {
        msg!("Checking Max Supply...");

        let vault_balance = self.vault_ata.amount;

        require!(vault_balance > 0, CustomError::MaxSupplyExceeded);

        msg!("Checking User Ownership...");

        let user_ata = get_associated_token_address(&user.key(), &self.mint.key());
        require!(
            self.token_program.exit(&user_ata).is_ok(),
            CustomError::UserAlreadyOwnsNft
        );

        msg!("Charging User...");

        let transfer_accounts = Transfer {
            from: user.to_account_info(),
            to: self.event_creator.to_account_info(),
            authority: user.clone(),
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer(cpi_context, self.event.ticket_price.into())?;

        msg!("Transferring NFT from Vault to User...");

        let transfer_accounts = Transfer {
            from: self.vault_ata.to_account_info(),
            to: user,
            authority: self.vault.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer(cpi_context, 1)?;

        msg!("Transfer Successful.");
        Ok(())
    }
}
