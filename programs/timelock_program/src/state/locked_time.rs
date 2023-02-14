use anchor_lang::prelude::*;

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum LockedTime {
    DevOnly,
    OneDay,
    TreeDays,
    OneWeek,
}

pub const ONE_DAY_IN_SECONDS: i64 = 60 * 60 * 24;

impl LockedTime {
    pub fn to_seconds(&self) -> i64 {
        match self {
            LockedTime::OneDay => ONE_DAY_IN_SECONDS,
            LockedTime::TreeDays => ONE_DAY_IN_SECONDS * 3,
            LockedTime::OneWeek => ONE_DAY_IN_SECONDS * 7,
            LockedTime::DevOnly => 10,
        }
    }
}

impl Default for LockedTime {
    fn default() -> Self {
        Self::TreeDays
    }
}
