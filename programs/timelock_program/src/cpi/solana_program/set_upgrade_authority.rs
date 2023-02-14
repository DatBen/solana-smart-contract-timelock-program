use anchor_lang::prelude::*;

pub fn handler<'info>(
    program: &AccountInfo<'info>,
    program_data: &AccountInfo<'info>,
    current_program_authority: &AccountInfo<'info>,
    new_authority: &AccountInfo<'info>,
) -> Result<()> {
    let ix = solana_program::bpf_loader_upgradeable::set_upgrade_authority(
        program.key,
        current_program_authority.key,
        Some(&new_authority.key),
    );
    solana_program::program::invoke(
        &ix,
        &[
            program_data.to_account_info(),
            current_program_authority.to_account_info(),
            new_authority.to_account_info(),
        ],
    )?;
    Ok(())
}
