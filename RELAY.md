# Relay Chain

In order to start a relaychain network you need to do some steps before:

- Compile `zkv-relay` and `paratest` nodes
- Generate relay chain spec file
- Generate parachain chain spec, genesis and wasm code
- Create the docker images for relay chain and parachain nodes
- Start compose file
- Start Parachain
  - By Initialize it
  - By Upgrade Runtime

## Compile `zkv-relay` and `paratest` nodes

```sh
cargo build --release -p zkv-relay --features "fast-runtime"
cargo build --release -p paratest-node
```

## Generate relay chain spec file

```sh
./target/release/zkv-relay build-spec \
        --chain local \
        --disable-default-bootnode > staging/plain-chainspec.json && \
    ./target/release/zkv-relay build-spec \
        --chain staging/plain-chainspec.json \
        --disable-default-bootnode --raw > staging/raw-chainspec.json
```

## Generate parachain chain spec, genesis and wasm code

```sh
./target/release/paratest-node build-spec \
        --chain local \
        --disable-default-bootnode > staging/plain-parachain-chainspec.json && \
    ./target/release/paratest-node build-spec \
        --chain staging/plain-parachain-chainspec.json \
        --disable-default-bootnode --raw > staging/raw-parachain-chainspec.json && \
    ./target/release/paratest-node export-genesis-state \
        --chain staging/raw-parachain-chainspec.json \
        staging/para-genesis-state && \
    ./target/release/paratest-node export-genesis-wasm \
        --chain staging/raw-parachain-chainspec.json \
        staging/para-wasm
```

## Create the docker images for relay chain and parachain nodes

```sh
docker/scripts/build-zkv-relay-image-injected.sh
docker/scripts/build-paratest-image-injected.sh
```

## Start compose file

```sh
docker compose -f docker/dockerfiles/zkv-relay-docker-compose.yaml up -d --remove-orphans
```

This compose starts 3 relaychain nodes and 3 parachain nodes:

- 2 relaychain validators (alice and bob)
- 2 parachain collators (alice and bob)
- 1 rpc gateway node for relay chain network that expose `9944` port for rpc and `30333` port for p2p
- 1 rpc gateway node for parachain network that expose `8844` port for rpc and `20333` port for p2p

## Start Parachain

### By Initialize it

Now the complete network is up, and we can initialize the parachain:

- Point polkadot.js to local chain at `ws://127.0.0.1:9944`
- Initialize parachain: _Developer_->_Sudo_->`parasSudoWrapper` pallet->`sudoScheduleParaInitialize` and set following data:
  - `id`: `1599`.
  - `genesisHead`: Click file upload and upload the genesis state file in `staging/para-genesis-state`.
  - `validationCode`: Click file upload and upload the WebAssembly runtime file in `staging/para-wasm`.
  - `paraKind`: Select `Yes`.

Now just wait (since 2 epochs/minutes) and the parchain should start to forge the blocks regularly every 12 seconds.

You can access to the parachain interface to point polkadot.js to `ws://localhost:8844`.

### By Upgrade Runtime

- Increase the runtime `spec_version` in `runtime/src/lib.rs`
- Convert parachain genesis state and wasm to binary format:
  
  ```sh
  cat staging/para-genesis-state | scripts/convert_hex_to_bytes.py > runtime/src/paratest_genesis
  cat staging/para-wasm | scripts/convert_hex_to_bytes.py > runtime/src/paratest_wasm 
  ```

- Compile the code with `add-parachain-upgrade` feature enable:
  
  ```sh
  cargo build --release -p nh-runtime --features "fast-runtime,add-parachain-upgrade"
  ```

- Upgrade runtime

## Extra

- To stop the chain use `docker compose -f docker/dockerfiles/zkv-relay-docker-compose.yaml down` with `-v` flag
  if you need also clear the chain
- To inspect the logs of a service use `docker compose -f docker/dockerfiles/zkv-relay-docker-compose.yaml logs <service>`
  where the available services are
  - `node_alice` relay chain alice validator
  - `node_bob` relay chain bob validator
  - `collator_alice` relay chain alice collator
  - `collator_bob` relay chain bob collator
  - `local_node` relay chain gateway node
  - `local_paranode` parachain gateway node
- To define node configurations and change the nodes log levels look at the files in
  `docker/resources/envs/[relay|para]` folders
