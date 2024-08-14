use anchor_lang::prelude::*;
use anchor_spl::{metadata::{mpl_token_metadata::instructions::{ ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts},
 MasterEditionAccount, Metadata, MetadataAccount}, 
 token::{ revoke, Mint, Revoke, Token, TokenAccount}};


use crate::state::{StakeAccount, StakeConfig, UserAccount};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,
   

    pub mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
       
        bump,
    )]
    pub metadata: Account<'info, MetadataAccount>, 

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]

    pub edition: Account<'info, MasterEditionAccount>, 

    pub config: Account<'info, StakeConfig>,


    #[account(
        mut,
        seeds =[b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,


    #[account(mut,
        close = user,
        seeds =[b"stake".as_ref(),  mint.key().as_ref(), config.key().as_ref()],
        bump = stake_account.bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {

 
      
        let mint = &self.mint.to_account_info();
        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        ThawDelegatedAccountCpi::new(
            metadata_program,
            ThawDelegatedAccountCpiAccounts{
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        ).invoke()?;

        
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts =  Revoke{
            source: self.mint_ata.to_account_info(),
            authority: self.stake_account.to_account_info(),
        };
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"stake",
            self.mint.to_account_info().key.as_ref(),
            self.config.to_account_info().key.as_ref(),
            &[self.stake_account.bump],
        ]];

        let cpi_context =  CpiContext::new_with_signer(
            cpi_program, cpi_accounts, &signer_seeds);

        revoke(cpi_context)?;

        self.user_account.amount_staked -= 1;

        let days_to_secs = 86400; // 24* 36000
        let time_elapsed = ((Clock::get()?.unix_timestamp - self.stake_account.last_update) / days_to_secs) as u32;
        
        self.user_account.points += time_elapsed as u32 * self.config.point_per_stake as u32;

        
        

        Ok(())
    }
}