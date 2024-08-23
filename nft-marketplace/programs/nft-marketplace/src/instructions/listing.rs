use anchor_lang::prelude::*;

use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{
    error::MarketplaceError,
    state::{Listing, Marketplace},
};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    maker: Signer<'info>,

    #[account(
       
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,

    maker_mint: Box<InterfaceAccount<'info, Mint>>,

    colection_mint: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        associated_token::authority = maker,
        associated_token::mint = maker_mint,
    )]
    maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init,
        payer = maker,
        space = Listing::INIT_SPACE,
        seeds = [marketplace.key().as_ref(), maker_mint.as_ref()],
        bump,
    )]
    listing: Box<InterfaceAccount<'info, Listing>>,

    #[account(
        init_if_needed,
        payer = maker,
        associated_token::authority = listing,
        associated_token::mint = maker_mint,
    )]
    vault: Box<Account<'info, TokenAccount>>,

    

}
