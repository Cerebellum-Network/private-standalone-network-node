# How to deploy

### Option 1: Run public docker image immediately with command:
```bash
docker run -d -P --name cerenode cerebellumnetwork/turnkey-private-blockchain-network
```

### Option 2: Build docker image locally from sources:
1. Clone this repository.
2. Run command to build locally:
```bash
docker build .
```

### Option 3: Run in Docker by docker compose
1. Clone this repository
2. Run the following command to start a single node development chain.
```bash
./scripts/docker_run.sh
```

# Option 4: Build from source locally without docker:
1. Clone this repository
2. Run command to build from sources
```bash
cargo +nightly-2020-10-06 build
```
3. Once it finished (it could take a while), you can run node with command:
```bash
./target/debug/node-template --dev --tmp
```
