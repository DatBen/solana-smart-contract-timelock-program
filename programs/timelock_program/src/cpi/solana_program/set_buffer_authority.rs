use anchor_lang::prelude::*;

pub fn without_signer_seed<'info>(
    buffer: &AccountInfo<'info>,
    current_buffer_authority: &AccountInfo<'info>,
    new_authority: &AccountInfo<'info>,
) -> Result<()> {
    let ix = solana_program::bpf_loader_upgradeable::set_buffer_authority(
        buffer.key,
        current_buffer_authority.key,
        new_authority.key,
    );
    solana_program::program::invoke(
        &ix,
        &[
            buffer.to_account_info(),
            current_buffer_authority.to_account_info(),
            new_authority.to_account_info(),
        ],
    )?;
    Ok(())
}

pub fn with_signer_seed<'info>(
    buffer: &AccountInfo<'info>,
    current_buffer_authority: &AccountInfo<'info>,
    new_authority: &AccountInfo<'info>,
    signer_seed: &[&[&[u8]]],
) -> Result<()> {
    let ix = solana_program::bpf_loader_upgradeable::set_buffer_authority(
        buffer.key,
        current_buffer_authority.key,
        new_authority.key,
    );
    solana_program::program::invoke_signed(
        &ix,
        &[
            buffer.to_account_info(),
            current_buffer_authority.to_account_info(),
            new_authority.to_account_info(),
        ],
        signer_seed,
    )?;
    Ok(())
}
