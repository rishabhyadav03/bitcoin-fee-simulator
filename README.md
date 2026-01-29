# Bitcoin Fee Simulator

A simple Rust CLI tool that simulates Bitcoin transaction fees using live
mempool data.

The tool helps estimate how much fee (in satoshis) a transaction should pay
based on its virtual size (vB) and desired confirmation priority.

## Features
- Fetches live fee rates from mempool.space
- Supports different confirmation targets (fast, medium, slow)
- Calculates total transaction fee in satoshis
- Clean, minimal CLI interface

## How it works
Bitcoin transaction fees are calculated using *sat/vB* (satoshis per virtual
byte). This tool:
1. Fetches recommended fee rates from the mempool
2. Selects a fee rate based on the chosen priority
3. Multiplies it by the transaction vsize to estimate total fee

## Usage

```bash
cargo run -- --vsize 140 --target fast
