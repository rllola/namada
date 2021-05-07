# Anoma ledger prototype

## Quick start

The ledger currently requires that [Tendermint version 0.34.x](https://github.com/tendermint/tendermint) is installed and available on path. [The pre-built binaries and the source for 0.34.8 are here](https://github.com/tendermint/tendermint/releases/tag/v0.34.8), also directly available in some package managers.

The transaction code can currently be built from [tx_template](txs/tx_template) and validity predicates from [vp_template](vps/vp_template), which is Rust code compiled to wasm.

The transaction template calls functions from the host environment. The validity predicate template can validate a transaction and the key changes that is has performed.

The validity predicate is currently hard-coded in the shell and used for every account. To experiment with a different validity predicate, build it from the template and restart the shell.

The gossip node needs to toggle the intent flag `--intent` to activate the intent broadcaster, multiple nodes can be connected with the `--peers` option.

The matchmaker template receives intents with the borsh encoding define in `data_template` and crafts data to be sent with `tx_intent_template` to the ledger. It matches only two intents that are the exact opposite.

```shell
# Install development dependencies
make dev-deps

# Run this first if you don't have Rust wasm target installed:
make -C txs/tx_template deps

# Build the validity predicate, transaction and matchmaker wasm modules:
make build-wasm-scripts

# Build Anoma
make

# Build and link the executables
make install

# generate default config in .anoma/
cargo run --bin anoman -- --base-dir .anoma generate-config

# Run Anoma node (this will also initialize and run Tendermint node)
make run-ledger

# Reset the state (resets Tendermint too)
cargo run --bin anoman -- reset-ledger

# Submit a custom transaction with a wasm code and arbitrary data in `tx.data` file.
# Note that you have to have a `tx.data` file for this to work, albeit it can be empty.
cargo run --bin anoma -- tx --code-path txs/tx_template/tx.wasm --data-path tx.data

# Setup temporary addresses aliases until we have a better client support
export ADA=a1qq5qqqqqg4znssfsgcurjsfhgfpy2vjyxy6yg3z98pp5zvp5xgersvfjxvcnx3f4xycrzdfkak0xhx
export ALAN=a1qq5qqqqqxv6yydz9xc6ry33589q5x33eggcnjs2xx9znydj9xuens3phxppnwvzpg4rrqdpswve4n9
export XAN=a1qq5qqqqqxuc5gvz9gycryv3sgye5v3j9gvurjv34g9prsd6x8qu5xs2ygdzrzsf38q6rss33xf42f3
export BTC=a1qq5qqqqq8q6yy3p4xyurys3n8qerz3zxxeryyv6rg4pnxdf3x3pyv32rx3zrgwzpxu6ny32r3laduc

# Submit a token transfer
cargo run --bin anomac -- transfer --source $ALAN --target $ADA --token $XAN --amount 10.1 --code-path txs/tx_transfer/tx.wasm

# Watch and on change run a node (the state will be persisted)
cargo watch -x "run --bin anoman -- run-ledger"

# Watch and on change reset & run a node
cargo watch -x "run --bin anoman -- reset-ledger" -x "run --bin anoman -- run"

# run gossip node with intent broadcaster and rpc server (use default config)
cargo run --bin anoma -- run-gossip --rpc

# run gossip node with intent broadcaster, matchmaker and rpc (use default config)
cargo run --bin anoman -- run-gossip --rpc --matchmaker-path matchmaker_template/matchmaker.wasm --tx-code-path txs/tx_from_intent/tx.wasm --ledger-address "127.0.0.1:26658"

# craft two opposite intents
cargo run --bin anomac -- craft-intent --address $ALAN --token-buy $BTC --amount-buy 20 --token-sell $XAN --amount-sell 10 --file-path intent_A.data
cargo run --bin anomac -- craft-intent --address $ADA --token-buy $XAN --amount-buy 10 --token-sell $BTC --amount-sell 20 --file-path intent_B.data

# Subscribe to new network
cargo run --bin anomac -- subscribe-topic --node "http://[::1]:39111" --topic "asset_v1"

# Submit the intents (need a rpc server), hardcoded address rpc node address
cargo run --bin anomac -- intent --node "http://[::1]:39111" --data-path intent_A.data --topic "asset_v1"
cargo run --bin anomac -- intent --node "http://[::1]:39111" --data-path intent_B.data --topic "asset_v1"

# Format the code
make fmt
```

## Logging

To change the log level, set `ANOMA_LOG` environment variable to one of:
- `error`
- `warn`
- `info`
- `debug`
- `trace`

To reduce amount of logging from Tendermint ABCI, which has a lot of `debug` logging, use e.g. `ANOMA_LOG=debug,tendermint_abci=warn`.
