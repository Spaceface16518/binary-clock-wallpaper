use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum TimeConvention {
    /// 24-hour time ("military" time)
    International,
    /// 12-hour time (conventional US/Canada time)
    Imperial,
}
