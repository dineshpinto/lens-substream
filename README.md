# Lens Substream

A Rusty way to stream Lens data using Substreams.

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Installation

1. [Copy this repo](https://github.com/dineshpinto/lens-substreams/generate)
2. Get an API key from [https://app.streamingfast.io/](https://app.streamingfast.io/)
3. Add the API key to your environment variables
```bash
export STREAMINGFAST_KEY='xxx'
export SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'$STREAMINGFAST_KEY'"}' | jq -r .token)
```
4. Install the [Substreams CLI](https://substreams.streamingfast.io/getting-started/installing-the-cli)
5. Generate protobuf: `make codegen`
6. Compile: `make build`
7. Start Substream (customize start/stop block in Makefile): `make stream`


## Limitations

- `Tuples` are not currently supported by ethabi (see [open issue](https://github.com/openethereum/ethabi/issues/175)),
  so they have been removed from the Lens Protocol Profiles (LPP) ABI. Currently, only implements protobuf interfaces for Lens collects and mirrors.
