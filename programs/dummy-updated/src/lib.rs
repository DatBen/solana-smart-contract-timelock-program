use anchor_lang::prelude::*;

declare_id!("8ABaFkrMCddzUAwSDcP48LoHjHbzMe8NwYq4H6STp66d");

#[program]
pub mod dummy {
    use super::*;

    pub fn first_ix(_ctx: Context<Ctx>) -> Result<()> {
        msg!("you called the first ix!");
        Ok(())
    }

    pub fn second_ix(_ctx: Context<Ctx>) -> Result<()> {
        msg!("you called the second ix!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Ctx {}
