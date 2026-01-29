use crate::models::FeeRates;

pub fn fetch_fee_rates() -> Result<FeeRates, reqwest::Error> {
    let url = "https://mempool.space/api/v1/fees/recommended";
    reqwest::blocking::get(url)?.json::<FeeRates>()
}
