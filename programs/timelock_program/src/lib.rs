use anchor_lang::prelude::*;
use ix::*;
use state::locked_time::LockedTime;

declare_id!("7tKeFVr5wPggkfQKwTqxt9P6Cd6cRRyMttjj3kT2bjFY");

pub mod cpi;
pub mod event;
pub mod ix;
pub mod state;
pub mod time_lock_error;

#[program]
pub mod timelock_program {

    use super::*;

    pub fn initialize_time_lock(ctx: Context<InitializeTimeLock>, timelock_bump: u8) -> Result<()> {
        ix::initialize_time_lock::handler(ctx, timelock_bump)
    }

    pub fn plan_update(
        ctx: Context<PlanUpdate>,
        locked_time: LockedTime,
        source_code_url: String,
    ) -> Result<()> {
        ix::plan_update::handler(ctx, locked_time, source_code_url)
    }

    pub fn commit_update(ctx: Context<CommitUpdate>) -> Result<()> {
        ix::commit_update::handler(ctx)
    }

    pub fn cancel_update(ctx: Context<CancelUpdate>) -> Result<()> {
        ix::cancel_update::handler(ctx)
    }
}

#[derive(Accounts)]
pub struct HashBenchmark {}
