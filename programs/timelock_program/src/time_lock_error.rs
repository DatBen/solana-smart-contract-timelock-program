use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum TimeLockError {
    #[msg("Update still locked")]
    UpdateLocked,

    #[msg("No update is planned")]
    NoUpdatePlanned,
}
