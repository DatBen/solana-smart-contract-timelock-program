use crate::{
    event::{commit_update_event::CommitUpdateEvent, plan_update_event::PlanUpdateEvent},
    time_lock_error::TimeLockError,
};

use super::locked_time::LockedTime;
use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct TimeLock {
    bump: [u8; 1],
    locked_program: Pubkey,
    timelock_admin: Pubkey,
    next_update: Option<Update>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Default, Clone)]
pub struct Update {
    update_locked_until: i64,
    new_data_buffer: Pubkey,
    source_code_url: String,
}

pub const BYTES_FOR_URL: usize = 150;

impl TimeLock {
    pub const LEN: usize = 8 + std::mem::size_of::<TimeLock>() + BYTES_FOR_URL;

    pub fn seed(&self) -> [&[u8]; 3] {
        [
            &b"timelock"[..],
            self.locked_program.as_ref(),
            self.bump.as_ref(),
        ]
    }

    pub fn initialize(
        &mut self,
        timelock_bump: u8,
        program_to_be_locked: Pubkey,
        timelock_admin: Pubkey,
    ) -> () {
        self.bump = [timelock_bump];
        self.locked_program = program_to_be_locked;
        self.timelock_admin = timelock_admin;
    }

    pub fn get_admin(&self) -> Pubkey {
        self.timelock_admin
    }

    pub fn plan_update(
        &mut self,
        locked_time: LockedTime,
        new_data_buffer: Pubkey,
        source_code_url: &String,
    ) -> Result<()> {
        let clock: Clock = Clock::get()?;
        self.next_update = Some(Update {
            new_data_buffer,
            update_locked_until: clock.unix_timestamp + locked_time.to_seconds(),
            source_code_url: source_code_url.to_owned(),
        });
        Ok(())
    }

    pub fn cancel_update(&mut self) {
        self.next_update = None;
    }

    pub fn allow_update(&self) -> Result<bool> {
        match &self.next_update {
            &Some(ref update) => {
                let clock: Clock = Clock::get()?;
                return Ok(clock.unix_timestamp.gt(&(update.update_locked_until - 1)));
            }
            &None => return err!(TimeLockError::NoUpdatePlanned),
        }
    }

    pub fn emit_plan_update_event(&self) -> Result<()> {
        match &self.next_update {
            &Some(ref next_update) => {
                emit!(PlanUpdateEvent {
                    program: self.locked_program,
                    next_update: next_update.clone(),
                });
                return Ok(());
            }
            &None => return err!(TimeLockError::NoUpdatePlanned),
        }
    }

    pub fn emit_commit_update_event(&self) -> Result<()> {
        return if !self.allow_update()? {
            err!(TimeLockError::UpdateLocked)
        } else {
            match &self.next_update {
                &Some(ref next_update) => {
                    emit!(CommitUpdateEvent {
                        program: self.locked_program,
                        update: next_update.clone(),
                    });
                    Ok(())
                }
                &None => err!(TimeLockError::NoUpdatePlanned),
            }
        };
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn url_size() {
        let url = String::from("https://github.com/coral-xyz/anchor/tree/v0.26.0");
        assert_eq!(url.len(), 48);
    }
}
