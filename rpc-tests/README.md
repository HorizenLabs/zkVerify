# Test suite for the NH-Core node RPC interface

## Installation

Issue the following commands:
```sh
npm install;
cp .env.testnet .env;
```

### Running the tests

To run the tests, ensure .env files exist then execute one of:
```sh
npm run test-local
npm run test-testnet
```

### Running with Docker

Build:
```sh
docker-compose build
```
Local Run:
```sh
docker-compose run -e TEST_ENV=local -e RPC_URL=http://local-rpc-url -e WEBSOCKET=ws://local-websocket -e PRIVATE_KEY=<local-private-key rpc-tests>
```
Testnet:
```sh
docker-compose run rpc-tests -e PRIVATE_KEY=<testnet-private-key rpc-tests>
```

### Running test for a specific namespace

You can run tests for a specific RPC namespace such as `rpc/chain` by issuing the following command:
```sh
npm run test-testnet rpc/chain;
```
### Running test for a specific RPC method

You can run tests for a specific RPC method such as `rpc/chain/getBlock` by issuing the following command:
```sh
npm run test-testnet rpc/chain/getBlock/index.test.ts
```

Have a look at the `rpc/` directory for the list of supported RPC methods that can be tested.

### Modifying the test parameters

You can modify the test parameters and use your own values by changing the .env values.
