use anchor_lang::{prelude::*, system_program};

// use anchor_spl::{associated_token, token};

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{self, Mint, Token, TokenAccount},
};
use mpl_token_metadata::accounts::{MasterEdition, Metadata as MetadataAccount};
// use mpl_token_metadata::{instructions as token_instruction, ID as TOKEN_METADATA_ID};

use crate::state::Ticket;

#[derive(Accounts)]
pub struct CreateNft<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = signer.key(),
        mint::freeze_authority = signer.key()
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub associated_token_account: Account<'info, TokenAccount>,
    pub collection: Account<'info, Mint>,

    // CHECK: Metaplex will check this
    #[account(
        mut,
        address = MetadataAccount::find_pda(&mint.key()).0,
    )]
    pub metadata_account: AccountInfo<'info>,
    // CHECK: Metaplex will check this
    #[account(
        mut,
        address = MasterEdition::find_pda(&mint.key()).0,
    )]
    pub master_edition_account: AccountInfo<'info>,
    pub ticket: Account<'info, Ticket>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateNft<'info> {
    pub fn create_nft(&mut self) -> Result<()> {
        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct CreateNft<'info> {
//     /// CHECK: Metaplex will check this
//     #[account(mut)]
//     pub metadata: UncheckedAccount<'info>,
//     // CHECK: Metaplex will check this
//     #[account(mut)]
//     pub master_edition: UncheckedAccount<'info>,

//     #[account(mut)]
//     pub mint: Signer<'info>,

//     #[account(mut)]
//     pub token_account: UncheckedAccount<'info>,

//     #[account(mut)]
//     pub mint_authority: Signer<'info>,
//     pub rent: Sysvar<'info, Rent>,
//     pub system_program: Program<'info, System>,
//     pub token_program: Program<'info, token::Token>,
//     pub associated_token_program: Program<'info, associated_token::AssociatedToken>,

//     pub token_metadata_program: UncheckedAccount<'info>,
// }
