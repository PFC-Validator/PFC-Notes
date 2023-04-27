

# PFC-Notes

A simple way to store notes on-chain.

## How to use
get a cron job, and use that to keep the notes fresh.

in your dApp, run queries like you normally would to access the notes, and make use of your chains RPC node infrastructure
or if your cheap, pick a testnet to store the notes.

## Don't be a dick. 

use this for low-traffic things, or run your own sentry nodes, utilize the IPFS url to store large things.

## Why ?
it was either this or buy some infrastructure to do caching, which isn't very decentralized of me. 

I thought I'd just use this 

## Warning / Disclaimer

no audits have been conducted, use at your own risk, stake on your local PFC node etc etc.

## How to Build

Install just: https://github.com/casey/just

Run linter:

```bash
just clippy
```

Run unit tests:

```bash
just test
```

Compile all contracts using [rust-optimizer](https://github.com/CosmWasm/rust-optimizer):

```bash
just optimize
```

## License

ASL 2.0

## Thanks to
[Larry0x's starter template](https://github.com/larry0x/cw-template)