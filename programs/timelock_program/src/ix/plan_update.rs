use anchor_lang::prelude::*;

use crate::state::{bpf::BpfLoaderUpgradeable, locked_time::LockedTime, timelock::TimeLock};

#[derive(Accounts)]
pub struct PlanUpdate<'info> {
    #[account(mut, address = timelock.get_admin())]
    pub current_authority: Signer<'info>,

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

pub fn handler(
    ctx: Context<PlanUpdate>,
    locked_time: LockedTime,
    source_code_url: String,
) -> Result<()> {
    ctx.accounts.timelock.plan_update(
        locked_time,
        ctx.accounts.new_program_data.key(),
        &source_code_url,
    )?;
    crate::cpi::solana_program::set_buffer_authority::without_signer_seed(
        &ctx.accounts.new_program_data,
        &ctx.accounts.current_authority,
        &ctx.accounts.timelock.to_account_info(),
    )?;
    ctx.accounts.timelock.emit_plan_update_event()?;
    Ok(())
}
