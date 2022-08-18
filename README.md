# squads-grinder

Inspired by `solana-keygen grind`, this is a grinder written in rust for vanity addresses on the Squads Multisig seen here: https://github.com/squads-dapp/squads-mpl

## Using this tool

Clone the repo, then run `cargo run --release <your-pattern> <thread-count?>` to grind for the pattern you want. The match is case-insensitive. Once found, the keypair json file will be written to your current directory. The thread count is optional and has 10 as default.
