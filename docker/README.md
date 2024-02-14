# Dockers and Scripts

We provide some scripts and Dockerfiles to simplify developing and testing.

## TLDR

Compile and start a cluster with 2 validators (Alice and Bob) and a simple client node with:

```bash
> . cfg
> bootstrap.sh
...
> docker compose -f docker/dockerfiles/hl-docker-compose.yaml up
```

## Scripts

```text
├── cfg                   # Add scripts path to your PATH environment variable
├── docker
│   └── scripts
│       ├── bootstrap.sh  # Compile and generate an image with injected node binary
│       ├── build-chain-image-injected.sh # Create an image with injected node binary
│       ├── build-injected.sh # Create an image with injected binary
│       └── my_cargo      # Use cargo from a docker image with all dependencies installed but
                          # leverage host environment for caching: useful to avoid
                          # problems related to incompatible glibc versions without lost 
                          # cache compilation
...
```

### `bootstrap.sh`

The simple workflow is:

```bash
> . cfg # Just the first time
> bootstrap.sh
```

After you'll have the `hl/nh-node` docker image on your local docker repository. You can run a _**solo**_ chain with

```bash
> docker run -ti --rm -p 9944:9944 hl/nh-node --dev --rpc-cors all --rpc-external
```

Where:

* `-p 9944:9944`: provide the access to the rpc interface on your host
* `--rpc-cors all --rpc-external`: enable the access from _polkadot.js_ by relaxing the cors policy

The `nh-node` binary is also available on your host environment at `target/release/nh-node`.

### `my_cargo`

`my_cargo` is a simple `cargo` replacement that execute the command inside a `rbuilder` docker container. `rbuilder` has all Rust's dependencies installed and use the host's environment to inherit cargo's repository cache, user github credentials to fetch the private repository and the local target folder to save binaries and incremental compilation artifacts.

### `build-injected.sh`

A script that generate a docker image with the base dependencies and the given executables injected. See `build-chain-image-injected.sh` as example of how to use it.

## Docker and Compose

All Dockerfile and compose definitions are located in `docker/dockerfile` folder.

* `hl-builder.Dockerfile`: create an image with all dependencies needed to compile the node and is used by `my_cargo` script
* `binary_injected.Dockerfile`: Is mainly used by the scripts and inject one or more binaries in a standard ubuntu 22:04 image
* `hl-node.Dockerfile`: generate a node image with a fresh source compilation (leverage on docker layers to create a small docker image)
* `hl-docker-compose.yaml`: the cluster definition that run
2 validator nodes (Alice and Bob) and a simple node that expose its rpc and P2P ports on localhost.

To generate a node image without bothering about local resources, Rust installation and so on you can simply use:

```bash
> docker build -f docker/dockerfiles/hl-node.Dockerfile -t nh-node:latest .
```

and run it with

```bash
> docker run -ti --rm nh-node --dev
```

All arguments after `nh-node` image name will be passed to the node executable.

### Compose

Some notes about compose cluster configuration: if you want to run a node with some specific environment variables you can just edit the files in `docker/resources/envs`.
