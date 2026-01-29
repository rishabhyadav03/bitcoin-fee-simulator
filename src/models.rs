use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FeeRates {
    pub fastestFee: u64,
    pub halfHourFee: u64,
    pub hourFee: u64,
    pub economyFee: u64,
    pub minimumFee: u64,
}
