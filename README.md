# Lens Substream

A Rusty way to stream Lens data using Substreams.

---

Decentralized social media generates a huge amount of on-chain data. This on-chain data needs a low-latency interface
for it to remain accessible for all, along with adding the ability to perform real-time analysis and monitoring.

Substreams are a data solution developed for The Graph Network. They allow developers to write Rust modules composing
data streams with low-cost caching and archiving of blockchain data, high throughput processing, and cursor-based reorgs
handling.

This project builds a (basic) Substream in Rust to monitor on-chain Lens protocol data and provide a low-latency data
streaming interface. Potential uses of such a tool could be for real-time analysis and monitoring of Lens data. Since
Substreams are streamed, the data can be directly injected into a variety of databases (BigQuery, Clickhouse,
Kafka etc.), opening up a lot of distinct analytics use cases.

A few unrelated citations:

1. [Gizmodo: Here Are 30 of the Hundreds of Subreddits Going Dark to Protest Reddit's API Changes](https://gizmodo.com/reddit-subreddit-api-r-gaming-1850510180)
2. [Verge: Twitter just closed the book on academic research](https://www.theverge.com/2023/5/31/23739084/twitter-elon-musk-api-policy-chilling-academic-research)
3. [9to5Google: Reddit’s API changes kill Android’s best third-party apps, including RIF and Sync](https://9to5google.com/2023/06/08/reddit-android-third-party-apps/)

## Installation

1. [Copy this repo](https://github.com/dineshpinto/lens-substreams/generate)
2. Get an API key from [StreamingFast](https://app.streamingfast.io/)
3. Install the [Substreams CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)
4. Add the API key to your environment variables

```bash
export STREAMINGFAST_KEY='xxx'
export SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'$STREAMINGFAST_KEY'"}' | jq -r .token)
```

### Using Makefile defaults

```bash
make codegen
make build
make stream
```

### Manual compilation

#### Generate protobuf

```bash
substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
```

#### Compile

```bash
cargo build --target wasm32-unknown-unknown --release
```

#### Start Substream

```bash
substreams run -e polygon.streamingfast.io:443 substreams.yaml map_transfers -s 43764445 -t 100
```

## Limitations

- `Tuples` are not currently supported by `rust-ethereum/ethabi` (
  see [open issue](https://github.com/openethereum/ethabi/issues/175)),
  so they have been removed from the Lens Protocol Profiles (LPP) ABI. Currently, only implements protobuf interfaces
  for Lens collects and mirrors.
