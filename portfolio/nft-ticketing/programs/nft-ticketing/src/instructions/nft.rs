use anchor_lang::{
    prelude::*,
    //  solana_program::program::invoke, system_program
};
use mpl_token_metadata::accounts::{MasterEdition, Metadata as MetadataAccount};
// anchor_spl::{
//     associated_token, token
// }

use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{Mint, Token, TokenAccount},
};
use crate::error::ErrorCode;
use crate::state::{Event, Ticket};

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
        associated_token::authority = signer,
    )]
    pub mint_ata: Account<'info, TokenAccount>,

     // CHECK: Metaplex will check this
    
    #[account(mut,
    address = MetadataAccount::find_pda(mint.key()).0
    
    )]
     // CHECK: Metaplex will check this
   
    pub metadata_account: AccountInfo<'info>,
    #[account(mut,
        address = MasterEdition::find_edition(mint.key()).0,
    )]
    pub master_edition_account: AccountInfo<'info>,
    pub ticket: Account<'info, Ticket>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
   
}
