use anchor_lang::prelude::*;

declare_id!("CNeP5kyAjtfH9gKrHq2bad9RdiJjYBB2Sme3yMTfe48Y");

#[program]
pub mod dummy {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("you called the old ix!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
