use anchor_lang::prelude::*;

use crate::{
    state::{bpf::BpfLoaderUpgradeable, timelock::TimeLock},
    time_lock_error::TimeLockError,
};

#[derive(Accounts)]
pub struct CommitUpdate<'info> {
    #[account(mut, address = timelock.get_admin())]
    pub timelock_admin: Signer<'info>,

    #[account(mut, executable, address = timelock.get_locked_program())]
    pub locked_program: AccountInfo<'info>,

    #[account(mut,
        seeds = [locked_program.key().as_ref()],
        bump,
        seeds::program = BpfLoaderUpgradeable::id()
    )]
    pub locked_program_data: AccountInfo<'info>,

    #[account(mut, address = timelock.get_next_update_buffer()?)]
    pub new_program_data: AccountInfo<'info>,

    #[account(
        seeds = [
            b"timelock".as_ref(),
            locked_program.key().as_ref()
        ],
        bump,
    )]
    pub timelock: Box<Account<'info, TimeLock>>,

    pub system_program: Program<'info, System>,

    pub bpf_upgradable_loader: Program<'info, BpfLoaderUpgradeable>,

    pub rent: Sysvar<'info, Rent>,

    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<CommitUpdate>) -> Result<()> {
    if !ctx.accounts.timelock.allow_update()? {
        return err!(TimeLockError::UpdateLocked);
    }
    crate::cpi::solana_program::upgrade_program_with_timelock::handler(
        &ctx.accounts.locked_program,
        &ctx.accounts.locked_program_data,
        &ctx.accounts.new_program_data,
        &ctx.accounts.timelock_admin,
        &ctx.accounts.timelock,
        &ctx.accounts.rent,
        &ctx.accounts.clock,
    )?;

    ctx.accounts.timelock.emit_commit_update_event()?;
    ctx.accounts.timelock.cancel_update();

    Ok(())
}
