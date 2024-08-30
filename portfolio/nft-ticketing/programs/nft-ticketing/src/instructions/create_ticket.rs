// use anchor_lang::{
//     prelude::*,
//     //  solana_program::program::invoke, system_program
// };

// // anchor_spl::{
// //     associated_token, token
// // }

// // mpl_token_metadata::{
// //     ID as TOKEN_METADATA_ID,
// //     instruction as token_instruction
// // }
// use crate::error::ErrorCode;
// use crate::state::Event;

// #[derive(Accounts)]
// #[instruction(event_name: String)]
// pub struct CreateTicket<'info> {
//     #[account(mut)]
//     pub organizer: Signer<'info>,

//     #[account(
//         init,
//         payer = organizer,
//         space = Event::INIT_SPACE,
//         seeds = [b"event", event_name.as_str().as_bytes()],
//         bump,
//     )]
//     pub event: Account<'info, Event>,
//     pub system_program: Program<'info, System>,
// }

// impl<'info> CreateEvent<'info> {
//     pub fn create_event(
//         &mut self,
//         event_name: String,
//         ticket_price: u16,
//         date: i64,
//         max_supply: u16,
//         description: String,
//         bumps: &CreateEventBumps,
//     ) -> Result<()> {
//         require!(max_supply > 0, ErrorCode::InvalidMaxSupply);
//         self.event.set_inner(Event {
//             organizer: self.organizer.key(),
//             ticket_price,
//             max_supply,
//             bump: bumps.event,
//             date,
//             event_name,
//             description,
//         });
//         Ok(())
//     }
// }

// // #[derive(Accounts)]
// // pub struct CreateEvent<'info> {
// //     #[account(mut)]
// //     pub maker: Signer<'info>,

// //     #[account(
// //         init,
// //         payer = maker,
// //         space = Event::INIT_SPACE,
// //         seeds = [b"event", event_name.as_str().as_bytes()],
// //         bump,
// //     )]
// //     pub event: Box<Account<'info, Event>>,
// //     pub maker_mint: Box<InterfaceAccount<'info, Mint>>,
// //     pub collection_mint: Box<InterfaceAccount<'info, Mint>>,

// //     #[account(
// //         mut,
// //         associated_token::authority = maker,
// //         associated_token::mint = maker_mint,
// //     )]
// //     pub maker_ata: Box<InterfaceAccount<'info, TokenAccount>>,
// //     /// CHECK: Metaplex will check this
// //     #[account(mut)]
// //     pub metadata_account: UncheckedAccount<'info>,
// //     pub metadata_program: Program<'info, Metadata>,
// //     pub associated_token_program: Program<'info, AssociatedToken>,
// //     /// CHECK: Metaplex will check this
// //     pub token_metadata_program: UncheckedAccount<'info>,
// //     pub token_program: Program<'info, token::Token>,
// //     pub system_program: Program<'info, System>,
// // }
