use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name mut be between 1 and 32 characters")]
    NameTooLong,
}
