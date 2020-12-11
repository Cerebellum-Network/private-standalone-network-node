# How to deploy

### Run public docker image
```bash
docker run -d -P --name cerenode cerebellumnetwork/turnkey-private-blockchain-network
```

### Build locally
```bash
docker build .
```

### Run in Docker by docker compose
First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Run Substrate node without re-compiling with rpc
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external --rpc-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

# Build from source:
```bash
cargo build --release
```

# Run for the development:
```bash
./target/release/node-template --dev --tmp
```
