use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum TimeConvention {
    /// 24-hour time ("military" time)
    International,
    /// 12-hour time (conventional US/Canada time)
    Imperial,
}

impl TimeConvention {
    /// conveniently convert from a boolean flag to a time convention,
    /// interpreting a boolean flag as a user choice to "use 12-hour time"
    /// (i.e. Imperial if true else International)
    pub fn from_use_12_flag(flag: bool) -> Self {
        if flag {
            TimeConvention::Imperial
        } else {
            TimeConvention::International
        }
    }
}
