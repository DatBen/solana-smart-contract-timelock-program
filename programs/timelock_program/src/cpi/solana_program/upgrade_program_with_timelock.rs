use anchor_lang::prelude::*;

use crate::state::timelock::TimeLock;

pub fn handler<'info>(
    locked_program: &AccountInfo<'info>,
    locked_program_data: &AccountInfo<'info>,
    new_program_data: &AccountInfo<'info>,
    current_authority: &AccountInfo<'info>,
    timelock: &Box<Account<'info, TimeLock>>,
    rent: &Sysvar<'info, Rent>,
    clock: &Sysvar<'info, Clock>,
) -> Result<()> {
    let ix = solana_program::bpf_loader_upgradeable::upgrade(
        locked_program.key,
        new_program_data.key,
        &timelock.key(),
        current_authority.key,
    );

    solana_program::program::invoke_signed(
        &ix,
        &[
            locked_program_data.to_account_info(),
            locked_program.to_account_info(),
            new_program_data.to_account_info(),
            current_authority.to_account_info(),
            rent.to_account_info(),
            clock.to_account_info(),
            timelock.to_account_info(),
        ],
        &[&timelock.seed()],
    )?;

    Ok(())
}
