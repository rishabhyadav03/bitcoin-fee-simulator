use crate::models::FeeRates;

pub fn calculate_fee(vsize: u64, rate: u64) -> u64 {
    vsize * rate
}

pub fn select_rate(fees: &FeeRates, target: &str) -> u64 {
    match target {
        "fast" => fees.fastestFee,
        "medium" => fees.halfHourFee,
        "slow" => fees.hourFee,
        _ => fees.minimumFee,
    }
}
