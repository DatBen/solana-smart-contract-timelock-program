use anchor_lang::prelude::*;

use crate::state::{bpf::BpfLoaderUpgradeable, timelock::TimeLock};

#[derive(Accounts)]
pub struct CancelUpdate<'info> {
    #[account(mut, address = timelock.get_admin())]
    pub timelock_admin: Signer<'info>,

    #[account(executable)]
    pub locked_program: AccountInfo<'info>,

    #[account(mut)]
    pub new_program_data: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"timelock".as_ref(),
            locked_program.key().as_ref()
        ],
        bump,
    )]
    pub timelock: Box<Account<'info, TimeLock>>,

    pub system_program: Program<'info, System>,

    pub bpf_upgradable_loader: Program<'info, BpfLoaderUpgradeable>,
}

pub fn handler(ctx: Context<CancelUpdate>) -> Result<()> {
    crate::cpi::solana_program::set_buffer_authority::with_signer_seed(
        &ctx.accounts.new_program_data,
        &ctx.accounts.timelock.to_account_info(),
        &ctx.accounts.timelock_admin,
        &[&ctx.accounts.timelock.seed()],
    )?;
    ctx.accounts.timelock.cancel_update();
    Ok(())
}
