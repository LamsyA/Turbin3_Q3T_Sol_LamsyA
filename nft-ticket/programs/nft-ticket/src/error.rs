use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The max supply of tickets has been exceeded.")]
    MaxSupplyExceeded,
    #[msg("The ticket is not listed for sale.")]
    TicketNotForSale,
    #[msg("Ticket for event must not be zero ")]
    InvalidMaxSupply,
}
