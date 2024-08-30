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
    // // CHECK: Metaplex will check this
    // #[account(mut)]
    // pub metadata: UncheckedAccount<'info>,
    // // CHECK: Metaplex will check this
    // #[account(mut)]
    // pub master_edition: UncheckedAccount<'info>,
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
    #[account(mut
        ,
    address = MetadataAccount::find_pda(mint.key()).0
    
    )]
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
    // #[account(mut)]
    // pub maker: Signer<'info>,

    // #[account(
    //     init,
    //     payer = maker,
    //     space = Event::INIT_SPACE,
    //     seeds = [b"event", event_name.as_str().as_bytes()],
    //     bump,
    // )]
    // pub event: Box<Account<'info, Event>>,
    // pub maker_mint: Box<InterfaceAccount<'info, Mint>>,
    // pub collection_mint: Box<InterfaceAccount<'info, Mint>>,

    // #[account(
    //     mut,
    //     associated_token::authority = maker,
    //     associated_token::mint = maker_mint,
    // )]
    // pub maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    // /// CHECK: Metaplex will check this
    // #[account(mut)]
    // pub metadata_account: UncheckedAccount<'info>,
    // pub metadata_program: Program<'info, Metadata>,
    // pub associated_token_program: Program<'info, AssociatedToken>,
    // /// CHECK: Metaplex will check this
    // pub token_metadata_program: UncheckedAccount<'info>,
    // pub token_program: Program<'info, token::Token>,
    // pub system_program: Program<'info, System>,
}
