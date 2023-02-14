use anchor_lang::prelude::*;

use crate::state::{bpf::BpfLoaderUpgradeable, timelock::TimeLock};

#[derive(Accounts)]
pub struct InitializeTimeLock<'info> {
    #[account(mut)]
    pub current_authority: Signer<'info>,

    #[account(mut, executable)]
    pub program_to_be_locked: AccountInfo<'info>,

    #[account(mut,
        seeds = [program_to_be_locked.key().as_ref()],
        bump,
        seeds::program = BpfLoaderUpgradeable::id()
    )]
    pub program_to_be_locked_data: AccountInfo<'info>,

    #[account(
        init,
        space = TimeLock::LEN,
        seeds = [
            b"timelock".as_ref(),
            program_to_be_locked.key().as_ref()
        ],
        bump,
        payer = current_authority
    )]
    pub timelock: Box<Account<'info, TimeLock>>,

    pub system_program: Program<'info, System>,

    pub bpf_upgradable_loader: Program<'info, BpfLoaderUpgradeable>,
}

pub fn handler(ctx: Context<InitializeTimeLock>, timelock_bump: u8) -> Result<()> {
    ctx.accounts.timelock.initialize(
        timelock_bump,
        ctx.accounts.program_to_be_locked.key(),
        ctx.accounts.current_authority.key(),
    );
    crate::cpi::solana_program::set_upgrade_authority::handler(
        &ctx.accounts.program_to_be_locked,
        &ctx.accounts.program_to_be_locked_data,
        &ctx.accounts.current_authority,
        &ctx.accounts.timelock.to_account_info(),
    )?;

    Ok(())
}
