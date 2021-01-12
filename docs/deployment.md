# How to deploy

### Option 1: Run public docker image
 You can run Private Node immediately. You don't even need to clone the repo. Use this command:
```bash
docker run -d --name NAME_OF_YOUR_NODE -p 9944:9944 --entrypoint /usr/local/bin/node-template cerebellumnetwork/turnkey-private-blockchain-network --dev --ws-external
```

### Option 2: Build docker image locally
In order to have a local Docker image, build from sources, you can use the following steps:
1. Clone this repository.
2. Run command to build locally (from the root directory of this repo):
```bash
docker build .
```

### Option 3: Build and run using docker-compose
In order to run ready-to-use Private Node, you can use `docker-compose` file provided by this repository with all pre-configured parameters. Follow the steps:
1. Clone this repository
2. Run command to start a single node development chain  (from the root directory of this repo):
```bash
./scripts/docker_run.sh
```

### Option 4: Build and run without docker
In order to build and run Private Node without docker you need to have `cargo` [installed and properly configured](https://doc.rust-lang.org/cargo/getting-started/installation.html). Follow the steps:
1. Clone this repository
2. Run command to build from sources (from the root directory of this repo)
```bash
cargo +nightly-2020-10-06 build
```
3. Once it finished (it could take a while), you can run node with command:
```bash
./target/debug/node-template --dev --tmp
```
*You can check more supported flags using command:*
```bash
./target/debug/node-template --help
```
