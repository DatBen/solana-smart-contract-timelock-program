use crate::state::timelock::Update;
use anchor_lang::prelude::*;

#[event]
pub struct CommitUpdateEvent {
    pub program: Pubkey,
    pub update: Update,
}
