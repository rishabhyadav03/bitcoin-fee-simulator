mod api;
mod logic;
mod models;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    vsize: u64,

    #[arg(long, default_value = "medium")]
    target: String,
}

fn main() {
    let args = Args::parse();

    let fees = api::fetch_fee_rates()
        .expect("Failed to fetch fee data");

    let rate = logic::select_rate(&fees, &args.target);
    let total_fee = logic::calculate_fee(args.vsize, rate);

    println!();
    println!("Transaction vsize : {} vB", args.vsize);
    println!("Fee rate          : {} sat/vB ({})", rate, args.target);
    println!("Total fee         : {} sats", total_fee);
}

