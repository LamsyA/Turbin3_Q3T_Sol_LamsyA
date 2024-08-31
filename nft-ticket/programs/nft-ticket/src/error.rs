use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("The max supply of tickets has been exceeded.")]
    MaxSupplyExceeded,
    #[msg("The ticket is not listed for sale.")]
    TicketNotForSale,
    #[msg("Ticket for event must not be zero ")]
    InvalidMaxSupply,
    #[msg("User already owns an NFT.")]
    UserAlreadyOwnsNft,
    #[msg("Event does not have tickets listed for sale.")]
    EventDoesNotHaveTickets,
    #[msg("Ticket price must not be zero.")]
    InvalidTicketPrice,
    #[msg("Event has reached its max supply.")]
    EventHasMaxSupply,
}
