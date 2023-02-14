use anchor_lang::prelude::*;

#[derive(Clone)]
pub struct BpfLoaderUpgradeable();

impl Id for BpfLoaderUpgradeable {
    fn id() -> Pubkey {
        solana_program::bpf_loader_upgradeable::id()
    }
}
