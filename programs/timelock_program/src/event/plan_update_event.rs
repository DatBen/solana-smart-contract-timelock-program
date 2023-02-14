use crate::state::timelock::Update;
use anchor_lang::prelude::*;

#[event]
pub struct PlanUpdateEvent {
    pub program: Pubkey,
    pub next_update: Update,
}
