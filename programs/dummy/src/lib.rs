use anchor_lang::prelude::*;

declare_id!("8ABaFkrMCddzUAwSDcP48LoHjHbzMe8NwYq4H6STp66d");

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
